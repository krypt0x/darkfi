use drk::{
    blockchain::{rocks::columns, Rocks, RocksColumn},
    cli::{Config, DarkfidConfig},
    client::{Client, State},
    crypto::{
        load_params, merkle::CommitmentTree, save_params, setup_mint_prover, setup_spend_prover,
    },
    rpc::{
        jsonrpc::{error as jsonerr, request as jsonreq, response as jsonresp, send_request},
        jsonrpc::{ErrorCode::*, JsonRequest, JsonResult},
        rpcserver::{listen_and_serve, RequestHandler, RpcServerConfig},
    },
    serial::{deserialize, serialize},
    util::{
        assign_id, decode_base10, encode_base10, expand_path, join_config_path, DrkTokenList,
        NetworkName, SolTokenList,
    },
    wallet::WalletDb,
    Error, Result,
};

use async_trait::async_trait;
use clap::clap_app;
use log::debug;
use serde_json::{json, Value};

use async_std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Cashier {
    pub name: String,
    pub rpc_url: String,
    pub public_key: jubjub::SubgroupPoint,
}

#[async_trait]
impl RequestHandler for Darkfid {
    async fn handle_request(&self, req: JsonRequest) -> JsonResult {
        if req.params.as_array().is_none() {
            return JsonResult::Err(jsonerr(InvalidParams, None, req.id));
        }

        debug!(target: "RPC", "--> {}", serde_json::to_string(&req).unwrap());

        match req.method.as_str() {
            Some("say_hello") => return self.say_hello(req.id, req.params).await,
            Some("create_wallet") => return self.create_wallet(req.id, req.params).await,
            Some("key_gen") => return self.key_gen(req.id, req.params).await,
            Some("get_key") => return self.get_key(req.id, req.params).await,
            Some("get_balances") => return self.get_balances(req.id, req.params).await,
            Some("get_token_id") => return self.get_token_id(req.id, req.params).await,
            Some("features") => return self.features(req.id, req.params).await,
            Some("deposit") => return self.deposit(req.id, req.params).await,
            Some("withdraw") => return self.withdraw(req.id, req.params).await,
            Some("transfer") => return self.transfer(req.id, req.params).await,
            Some(_) | None => return JsonResult::Err(jsonerr(MethodNotFound, None, req.id)),
        };
    }
}

struct Darkfid {
    client: Arc<Mutex<Client>>,
    sol_tokenlist: SolTokenList,
    drk_tokenlist: DrkTokenList,
    cashiers: Vec<Cashier>,
}

impl Darkfid {
    async fn new(client: Arc<Mutex<Client>>, cashiers: Vec<Cashier>) -> Result<Self> {
        let sol_tokenlist = SolTokenList::new()?;
        let drk_tokenlist = DrkTokenList::new(sol_tokenlist.clone())?;

        Ok(Self {
            client,
            sol_tokenlist,
            drk_tokenlist,
            cashiers,
        })
    }

    async fn start(&mut self, state: Arc<Mutex<State>>) -> Result<()> {
        self.client.lock().await.start().await?;
        self.client
            .lock()
            .await
            .connect_to_subscriber(state)
            .await?;

        Ok(())
    }

    // --> {"method": "say_hello", "params": []}
    // <-- {"result": "hello world"}
    async fn say_hello(&self, id: Value, _params: Value) -> JsonResult {
        JsonResult::Resp(jsonresp(json!("hello world"), id))
    }

    // --> {"method": "create_wallet", "params": []}
    // <-- {"result": true}
    async fn create_wallet(&self, id: Value, _params: Value) -> JsonResult {
        match self.client.lock().await.init_db().await {
            Ok(()) => return JsonResult::Resp(jsonresp(json!(true), id)),
            Err(e) => {
                return JsonResult::Err(jsonerr(ServerError(-32001), Some(e.to_string()), id))
            }
        }
    }

    // --> {"method": "key_gen", "params": []}
    // <-- {"result": true}
    async fn key_gen(&self, id: Value, _params: Value) -> JsonResult {
        match self.client.lock().await.key_gen().await {
            Ok(()) => return JsonResult::Resp(jsonresp(json!(true), id)),
            Err(e) => {
                return JsonResult::Err(jsonerr(ServerError(-32002), Some(e.to_string()), id))
            }
        }
    }

    // --> {"method": "get_key", "params": []}
    // <-- {"result": "vdNS7oBj7KvsMWWmo9r96SV4SqATLrGsH2a3PGpCfJC"}
    async fn get_key(&self, id: Value, _params: Value) -> JsonResult {
        let pk = self.client.lock().await.main_keypair.public;
        let b58 = bs58::encode(serialize(&pk)).into_string();
        return JsonResult::Resp(jsonresp(json!(b58), id));
    }

    // --> {"method": "get_balances", "params": []}
    // <-- {"result": "get_balances": "[ {"btc": (value, network)}, .. ]"}
    async fn get_balances(&self, id: Value, _params: Value) -> JsonResult {
        let result: Result<HashMap<String, (String, String)>> = async {
            let balances = self.client.lock().await.get_balances().await?;
            let mut symbols: HashMap<String, (String, String)> = HashMap::new();

            for (id, value) in balances.iter() {
                let id: jubjub::Fr = deserialize(&id)?;

                // this is hardcoded for SOL
                // TODO: if id == btc_id:
                //          network = bitcoin
                //      else
                //          network = solana

                let network = "solana";

                if let Some(symbol) = self.drk_tokenlist.symbol_from_id(id)? {
                    let amount = encode_base10(*value, 8);
                    symbols.insert(symbol, (amount, network.to_string()));
                }
            }
            Ok(symbols)
        }
        .await;
        match result {
            Ok(res) => JsonResult::Resp(jsonresp(json!(res), json!(res))),
            Err(err) => JsonResult::Err(jsonerr(InternalError, Some(err.to_string()), json!(id))),
        }
    }

    // --> {"method": "get_token_id", "params": [network, token]}
    // <-- {"result": "Ht5G1RhkcKnpLVLMhqJc5aqZ4wYUEbxbtZwGCVbgU7DL"}
    async fn get_token_id(&self, id: Value, params: Value) -> JsonResult {
        let args = params.as_array();

        if args.is_none() {
            return JsonResult::Err(jsonerr(InvalidParams, None, id));
        }

        let args = args.unwrap();

        let network = args[0].as_str();
        let symbol = args[1].as_str();

        if network.is_none() {
            return JsonResult::Err(jsonerr(InvalidNetworkParam, None, id));
        }

        if symbol.is_none() {
            return JsonResult::Err(jsonerr(InvalidSymbolParam, None, id));
        }
        let symbol = symbol.unwrap();

        let result: Result<Value> = async {
            let network = NetworkName::from_str(&network.unwrap())?;
            match network {
                #[cfg(feature = "sol")]
                NetworkName::Solana => {
                    let token_id = self.sol_tokenlist.search_id(symbol)?;
                    Ok(json!(token_id))
                }
                #[cfg(feature = "btc")]
                NetworkName::Bitcoin => {
                    return Err(Error::NotSupportedToken);
                }
                _ => Err(Error::NotSupportedNetwork),
            }
        }
        .await;

        match result {
            Ok(res) => JsonResult::Resp(jsonresp(json!(res), json!(res))),
            Err(err) => JsonResult::Err(jsonerr(InternalError, Some(err.to_string()), json!(id))),
        }
    }

    // --> {""method": "features", "params": []}
    // <-- {"result": { "network": ["btc", "sol"] } }
    async fn features(&self, id: Value, _params: Value) -> JsonResult {
        let req = jsonreq(json!("features"), json!([]));
        let rep: JsonResult;
        // TODO: this just selects the first cashier in the list
        match send_request(&self.cashiers[0].rpc_url, json!(req)).await {
            Ok(v) => rep = v,
            Err(e) => {
                return JsonResult::Err(jsonerr(ServerError(-32004), Some(e.to_string()), id))
            }
        }

        match rep {
            JsonResult::Resp(r) => return JsonResult::Resp(r),
            JsonResult::Err(e) => return JsonResult::Err(e),
            JsonResult::Notif(_) => return JsonResult::Err(jsonerr(InternalError, None, id)),
        }
    }

    // --> {"method": "deposit", "params": [network, token, publickey]}
    // The publickey sent here is used so the cashier can know where to send
    // tokens once the deposit is received.
    // <-- {"result": "Ht5G1RhkcKnpLVLMhqJc5aqZ4wYUEbxbtZwGCVbgU7DL"}
    async fn deposit(&self, id: Value, params: Value) -> JsonResult {
        let args = params.as_array();

        if args.is_none() {
            return JsonResult::Err(jsonerr(InvalidParams, None, id));
        }

        let args = args.unwrap();
        if args.len() != 2 {
            return JsonResult::Err(jsonerr(InvalidParams, None, id));
        }

        let network = &args[0];
        let token = &args[1];

        if token.as_str().is_none() {
            return JsonResult::Err(jsonerr(InvalidTokenIdParam, None, id));
        }

        let token = token.as_str().unwrap();

        if network.as_str().is_none() {
            return JsonResult::Err(jsonerr(InvalidNetworkParam, None, id));
        }

        let network = network.as_str().unwrap();

        let token_id = match assign_id(&network, &token, &self.sol_tokenlist) {
            Ok(t) => t,
            Err(e) => {
                return JsonResult::Err(jsonerr(InternalError, Some(e.to_string()), id));
            }
        };

        // TODO: Optional sanity checking here, but cashier *must* do so too.

        let pk = self.client.lock().await.main_keypair.public;
        let pubkey = bs58::encode(serialize(&pk)).into_string();

        // Send request to cashier. If the cashier supports the requested network
        // (and token), it shall return a valid address where tokens can be deposited.
        // If not, an error is returned, and forwarded to the method caller.
        let req = jsonreq(json!("deposit"), json!([network, token_id, pubkey]));
        let rep: JsonResult;
        match send_request(&self.cashiers[0].rpc_url, json!(req)).await {
            Ok(v) => rep = v,
            Err(e) => {
                debug!(target: "DARKFID", "REQUEST IS ERR");
                return JsonResult::Err(jsonerr(ServerError(-32004), Some(e.to_string()), id));
            }
        }

        match rep {
            JsonResult::Resp(r) => return JsonResult::Resp(r),
            JsonResult::Err(e) => return JsonResult::Err(e),
            JsonResult::Notif(_n) => return JsonResult::Err(jsonerr(InternalError, None, id)),
        }
    }

    // --> {"method": "withdraw", "params": [network, token, publickey, amount]}
    // The publickey sent here is the address where the caller wants to receive
    // the tokens they plan to withdraw.
    // On request, send request to cashier to get deposit address, and then transfer
    // dark tokens to the cashier's wallet. Following that, the cashier should return
    // a transaction ID of them sending the funds that are requested for withdrawal.
    // <-- {"result": "txID"}
    async fn withdraw(&self, id: Value, params: Value) -> JsonResult {
        let args = params.as_array();

        if args.is_none() {
            return JsonResult::Err(jsonerr(InvalidParams, None, id));
        }

        let args = args.unwrap();

        if args.len() != 4 {
            return JsonResult::Err(jsonerr(InvalidParams, None, id));
        }

        let network = &args[0];
        let token = &args[1];
        let address = &args[2];
        let amount = &args[3];

        if token.as_str().is_none() {
            return JsonResult::Err(jsonerr(InvalidTokenIdParam, None, id));
        }

        let token = token.as_str().unwrap();

        if network.as_str().is_none() {
            return JsonResult::Err(jsonerr(InvalidNetworkParam, None, id));
        }

        let network = network.as_str().unwrap();

        if amount.as_str().is_none() {
            return JsonResult::Err(jsonerr(InvalidNetworkParam, None, id));
        }

        let amount = amount.as_str().unwrap();

        let amount_in_apo = match decode_base10(&amount, 8, true) {
            Ok(a) => a,
            Err(e) => {
                return JsonResult::Err(jsonerr(InvalidAmountParam, Some(e.to_string()), id));
            }
        };

        let token_id = match assign_id(&network, &token, &self.sol_tokenlist) {
            Ok(t) => t,
            Err(e) => {
                return JsonResult::Err(jsonerr(InternalError, Some(e.to_string()), id));
            }
        };

        let req = jsonreq(
            json!("withdraw"),
            json!([network, token_id, address, amount_in_apo]),
        );
        let mut rep: JsonResult;
        match send_request(&self.cashiers[0].rpc_url, json!(req)).await {
            Ok(v) => rep = v,
            Err(e) => {
                return JsonResult::Err(jsonerr(ServerError(-32004), Some(e.to_string()), id));
            }
        }

        let token_id: &jubjub::Fr;

        // get the id for the token
        if let Some(tk_id) = self.drk_tokenlist.tokens.get(&token.to_uppercase()) {
            token_id = tk_id;
        } else {
            return JsonResult::Err(jsonerr(InvalidParams, None, id));
        }

        // send drk to cashier_public
        if let JsonResult::Resp(cashier_public) = &rep {
            let result: Result<()> = async {
                let cashier_public = cashier_public.result.as_str().unwrap();

                let cashier_public: jubjub::SubgroupPoint =
                    deserialize(&bs58::decode(cashier_public).into_vec()?)?;

                self.client
                    .lock()
                    .await
                    .transfer(token_id.clone(), cashier_public, amount_in_apo)
                    .await?;

                Ok(())
            }
            .await;

            match result {
                Err(e) => {
                    rep = JsonResult::Err(jsonerr(InternalError, Some(e.to_string()), id.clone()))
                }
                Ok(_) => {
                    rep = JsonResult::Resp(jsonresp(
                        json!(format!(
                            "Sent request to withdraw {} amount of {}",
                            amount, token_id
                        )),
                        json!(id.clone()),
                    ))
                }
            }
        };

        match rep {
            JsonResult::Resp(r) => return JsonResult::Resp(r),
            JsonResult::Err(e) => return JsonResult::Err(e),
            JsonResult::Notif(_n) => return JsonResult::Err(jsonerr(InternalError, None, id)),
        }
    }

    // --> {"method": "transfer", [dToken, address, amount]}
    // <-- {"result": "txID"}
    async fn transfer(&self, id: Value, params: Value) -> JsonResult {
        let args = params.as_array();
        if args.is_none() {
            return JsonResult::Err(jsonerr(InvalidParams, None, id));
        }
        let args = args.unwrap();
        if args.len() != 3 {
            return JsonResult::Err(jsonerr(InvalidParams, None, id));
        }

        let token = &args[0].as_str();
        let address = &args[1].as_str();
        let amount = &args[2].as_str();

        if token.is_none() {
            return JsonResult::Err(jsonerr(InvalidTokenIdParam, None, id));
        }

        let token = token.unwrap();

        if address.is_none() {
            return JsonResult::Err(jsonerr(InvalidAddressParam, None, id));
        }

        let address = address.unwrap();

        if amount.is_none() {
            return JsonResult::Err(jsonerr(InvalidAmountParam, None, id));
        }
        let amount = amount.unwrap();

        let token_id: &jubjub::Fr;

        // get the id for the token
        if let Some(tk_id) = self.drk_tokenlist.tokens.get(&token.to_uppercase()) {
            token_id = tk_id;
        } else {
            return JsonResult::Err(jsonerr(InvalidParams, None, id));
        }

        let result: Result<()> = async {
            let drk_address = bs58::decode(&address).into_vec()?;
            let drk_address: jubjub::SubgroupPoint = deserialize(&drk_address)?;

            let decimals: usize = 8;
            let amount = decode_base10(&amount, decimals, true)?;

            self.client
                .lock()
                .await
                .transfer(token_id.clone(), drk_address, amount)
                .await?;

            Ok(())
        }
        .await;

        match result {
            Ok(msg) => JsonResult::Resp(jsonresp(json!(msg), json!(id))),
            Err(err) => JsonResult::Err(jsonerr(InternalError, Some(err.to_string()), json!(id))),
        }
    }
}

#[async_std::main]
async fn main() -> Result<()> {
    let args = clap_app!(darkfid =>
        (@arg CONFIG: -c --config +takes_value "Sets a custom config file")
        (@arg verbose: -v --verbose "Increase verbosity")
    )
    .get_matches();

    let config_path = if args.is_present("CONFIG") {
        PathBuf::from(args.value_of("CONFIG").unwrap())
    } else {
        join_config_path(&PathBuf::from("darkfid.toml"))?
    };

    let loglevel = if args.is_present("verbose") {
        log::Level::Debug
    } else {
        log::Level::Info
    };

    simple_logger::init_with_level(loglevel)?;

    let config: DarkfidConfig = Config::<DarkfidConfig>::load(config_path)?;

    let wallet = WalletDb::new(
        expand_path(&config.wallet_path)?.as_path(),
        config.wallet_password.clone(),
    )?;

    let rocks = Rocks::new(expand_path(&config.database_path.clone())?.as_path())?;

    let mut cashiers = Vec::new();
    let mut cashier_keys = Vec::new();

    for cashier in config.clone().cashiers {

        if cashier.public_key.is_empty() {
            return Err(Error::CashierKeysNotFound);
        }

        let cashier_public: jubjub::SubgroupPoint =
            deserialize(&bs58::decode(cashier.public_key).into_vec()?)?;

        cashiers.push(Cashier {
            name: cashier.name,
            rpc_url: cashier.rpc_url,
            public_key: cashier_public,
        });

        cashier_keys.push(cashier_public);
    }

    // Load trusted setup parameters
    let params_paths = (
        expand_path(&config.mint_params_path.clone())?,
        expand_path(&config.spend_params_path.clone())?,
    );
    let mint_params_path = params_paths.0.to_str().unwrap_or("mint.params");
    let spend_params_path = params_paths.1.to_str().unwrap_or("spend.params");
    // Auto create trusted ceremony parameters if they don't exist
    if !params_paths.0.exists() {
        let params = setup_mint_prover();
        save_params(mint_params_path, &params)?;
    }
    if !params_paths.1.exists() {
        let params = setup_spend_prover();
        save_params(spend_params_path, &params)?;
    }
    let (mint_params, mint_pvk) = load_params(mint_params_path)?;
    let (spend_params, spend_pvk) = load_params(spend_params_path)?;


    let client = Client::new(
        rocks.clone(),
        (
            config.gateway_protocol_url.parse()?,
            config.gateway_publisher_url.parse()?,
        ),
        wallet.clone(),
        mint_params,
        spend_params,
    )
    .await?;

    let client = Arc::new(Mutex::new(client));

    let mut darkfid = Darkfid::new(client, cashiers).await?;

    let merkle_roots = RocksColumn::<columns::MerkleRoots>::new(rocks.clone());
    let nullifiers = RocksColumn::<columns::Nullifiers>::new(rocks);

    let state = Arc::new(Mutex::new(State {
        tree: CommitmentTree::empty(),
        merkle_roots,
        nullifiers,
        mint_pvk,
        spend_pvk,
        public_keys: cashier_keys,
    }));

    let server_config = RpcServerConfig {
        socket_addr: config.rpc_listen_address.clone(),
        use_tls: config.serve_tls,
        identity_path: expand_path(&config.tls_identity_path.clone())?,
        identity_pass: config.tls_identity_password.clone(),
    };

    darkfid.start(state).await?;
    listen_and_serve(server_config, Arc::new(darkfid)).await
}
