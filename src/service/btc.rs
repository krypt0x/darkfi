use super::bridge::CoinClient;
use crate::serial::{Decodable, Encodable, serialize};
use crate::Result;

use async_trait::async_trait;
use bitcoin::blockdata::script::Script;
use bitcoin::network::constants::Network;
use bitcoin::util::address::Address;
use bitcoin::util::ecdsa::{PrivateKey, PublicKey};
use bitcoin::hash_types::Txid;
use electrum_client::{Client as ElectrumClient, ElectrumApi};
use log::*;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use secp256k1::key::SecretKey;

use async_std::sync::Arc;
use std::str::FromStr;

// Swap out these types for any future non bitcoin-rs types
pub type PubAddress = Address;
pub type PubKey = PublicKey;
pub type PrivKey = PrivateKey;

pub struct BitcoinKeys {
    _secret_key: SecretKey,
    bitcoin_private_key: PrivateKey,
    btc_client: Arc<ElectrumClient>,
    pub bitcoin_public_key: PublicKey,
    pub pub_address: Address,
    pub script: Script,
    pub network: Network,
}

impl BitcoinKeys {
    pub fn new(
        btc_client: Arc<ElectrumClient>,
        network: Network,
    ) -> Result<Arc<BitcoinKeys>> {
        let context = secp256k1::Secp256k1::new();

        // Probably not good enough for release
        let rand: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect();

        let rand_hex = hex::encode(rand);

        // Generate simple byte array from rand
        let data_slice: &[u8] = rand_hex.as_bytes();

        let secret_key = SecretKey::from_slice(&hex::decode(data_slice).unwrap()).unwrap();

        // Use Testnet
        let bitcoin_private_key = PrivateKey::new(secret_key, network);

        let bitcoin_public_key = PublicKey::from_private_key(&context, &bitcoin_private_key);
        //let pubkey_serialized = bitcoin_public_key.to_bytes();

        let pub_address = Address::p2pkh(&bitcoin_public_key, network);

        let script = Script::new_p2pk(&bitcoin_public_key);

        Ok(Arc::new(BitcoinKeys {
            _secret_key: secret_key,
            bitcoin_private_key,
            btc_client,
            bitcoin_public_key,
            pub_address,
            script,
            network,
        }))
    }

    pub async fn start_subscribe(self: Arc<Self>) -> BtcResult<(Txid, u64)> {
        debug!(target: "BTC CLIENT", "Subscribe to scriptpubkey");
        let client = &self.btc_client;
        // Check if script is already subscribed
        if let Some(status_start) = client.script_subscribe(&self.script)? {
            loop {
                match client.script_pop(&self.script)? {
                    Some(status) => {
                        // Script has a notification update
                        if status != status_start {
                            let balance = client.script_get_balance(&self.script)?;
                            if balance.confirmed > 0 {
                                debug!(target: "BTC CLIENT", "BTC Balance: Confirmed!");
                                let history = client.script_get_history(&self.script)?;
                                //return tx_hash of latest tx that created balance
                                return Ok((history[0].tx_hash, balance.confirmed));
                            } else {
                                debug!(target: "BTC CLIENT", "BTC Balance: Unconfirmed!");
                                continue;
                            }
                        } else {
                            debug!(target: "BTC CLIENT", "ScriptPubKey status has not changed");
                            continue;
                        }
                    }
                    None => {
                        debug!(target: "BTC CLIENT", "Scriptpubkey does not yet exist in script notifications!");
                        continue;
                    }
                };
            } // Endloop
        } else {
            return Err(BtcFailed::ElectrumError(
                "Did not subscribe to scriptpubkey".to_string(),
            ));
        }
    }

    // This should do a db lookup to return the same obj
    pub fn address_from_slice(key: &[u8]) -> Result<Address> {
        let pub_key = PublicKey::from_slice(key).unwrap();
        let address = Address::p2pkh(&pub_key, Network::Testnet);

        Ok(address)
    }

    // This should do a db lookup to return the same obj
    pub fn private_key_from_slice(key: &[u8]) -> Result<PrivKey> {
        let key = PrivKey::from_slice(key, Network::Testnet).unwrap();
        Ok(key)
    }

    pub fn get_deposit_address(&self) -> Result<&Address> {
        Ok(&self.pub_address)
    }
    pub fn get_pubkey(&self) -> &PublicKey {
        &self.bitcoin_public_key
    }
    pub fn get_privkey(&self) -> &PrivateKey {
        &self.bitcoin_private_key
    }
    pub fn get_script(&self) -> &Script {
        &self.script
    }

}


pub struct BtcClient {
    client: Arc<ElectrumClient>,
    network: Network,
}

impl BtcClient {
    pub fn new( btc_endpoint: (bool, String) ) -> Result<Self> {
        let (network, client_address) = btc_endpoint;
        let client = ElectrumClient::new(&client_address)
            .map_err(|err| crate::Error::from(super::BtcFailed::from(err)))?;
        Ok(Self {
            client: Arc::new(client),
            network: if network {Network::Testnet} else {Network::Bitcoin},
        })
    }
}

#[async_trait]
impl CoinClient for BtcClient {
    async fn watch(&self) -> Result<(Vec<u8>, Vec<u8>)> {
        //// Generate bitcoin Address
        let btc_keys = BitcoinKeys::new(self.client.clone(), self.network)?;

        let btc_pub = btc_keys.clone();
        let btc_pub = btc_pub.get_pubkey();
        let btc_priv = btc_keys.clone();
        let btc_priv = btc_priv.get_privkey();

        // start scheduler for checking balance
        debug!(target: "BRIDGE BITCOIN", "Subscribing for deposit");

        let (_txid, _balance) = btc_keys.start_subscribe().await?;
        //let _script = btc_keys.get_script();

        Ok((serialize(&btc_priv.to_bytes()), serialize(&btc_pub.to_bytes())))
    }

    async fn send(&self, _address: Vec<u8>, _amount: u64) -> Result<()> {
        // TODO
        Ok(())
    }
}

impl Encodable for bitcoin::Address {
    fn encode<S: std::io::Write>(&self, s: S) -> Result<usize> {
        let addr = self.to_string();
        let len = addr.encode(s)?;
        Ok(len)
    }
}

impl Decodable for bitcoin::Address {
    fn decode<D: std::io::Read>(mut d: D) -> Result<Self> {
        let addr: String = Decodable::decode(&mut d)?;
        let addr = bitcoin::Address::from_str(&addr)
            .map_err(|err| crate::Error::from(BtcFailed::from(err)))?;
        Ok(addr)
    }
}

impl Encodable for bitcoin::PublicKey {
    fn encode<S: std::io::Write>(&self, s: S) -> Result<usize> {
        let key = self.to_bytes();
        let len = key.encode(s)?;
        Ok(len)
    }
}

impl Decodable for bitcoin::PublicKey {
    fn decode<D: std::io::Read>(mut d: D) -> Result<Self> {
        let key: Vec<u8> = Decodable::decode(&mut d)?;
        let key = bitcoin::PublicKey::from_slice(&key)
            .map_err(|err| crate::Error::from(BtcFailed::from(err)))?;
        Ok(key)
    }
}

impl Encodable for bitcoin::PrivateKey {
    fn encode<S: std::io::Write>(&self, s: S) -> Result<usize> {
        let key = self.to_bytes();
        let len = key.encode(s)?;
        Ok(len)
    }
}

impl Decodable for bitcoin::PrivateKey {
    fn decode<D: std::io::Read>(mut d: D) -> Result<Self> {
        let key: Vec<u8> = Decodable::decode(&mut d)?;
        let key = bitcoin::PrivateKey::from_slice(&key, Network::Testnet)
            .map_err(|err| crate::Error::from(BtcFailed::from(err)))?;
        Ok(key)
    }
}


#[derive(Debug)]
pub enum BtcFailed {
    NotEnoughValue(u64),
    BadBTCAddress(String),
    ElectrumError(String),
    BtcError(String),
    DecodeAndEncodeError(String),
}

impl std::error::Error for BtcFailed {}

impl std::fmt::Display for BtcFailed {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BtcFailed::NotEnoughValue(i) => {
                write!(f, "There is no enough value {}", i)
            }
            BtcFailed::BadBTCAddress(ref err) => {
                write!(f, "Unable to create Electrum Client: {}", err)
            }
            BtcFailed::ElectrumError(ref err) => write!(f, "could not parse BTC address: {}", err),
            BtcFailed::DecodeAndEncodeError(ref err) => write!(f, "Decode and decode keys error: {}", err),
            BtcFailed::BtcError(i) => {
                write!(f, "BtcFailed: {}", i)
            }
        }
    }
}


impl From<crate::error::Error> for BtcFailed {
    fn from(err: crate::error::Error) -> BtcFailed {
        BtcFailed::BtcError(err.to_string())
    }
}

impl From<bitcoin::util::address::Error> for BtcFailed {
    fn from(err: bitcoin::util::address::Error) -> BtcFailed {
        BtcFailed::BadBTCAddress(err.to_string())
    }
}
impl From<electrum_client::Error> for BtcFailed {
    fn from(err: electrum_client::Error) -> BtcFailed {
        BtcFailed::ElectrumError(err.to_string())
    }
}

impl From<bitcoin::util::key::Error> for BtcFailed {
    fn from(err: bitcoin::util::key::Error) -> BtcFailed {
        BtcFailed::DecodeAndEncodeError(err.to_string())
    }
}

pub type BtcResult<T> = std::result::Result<T, BtcFailed>;

#[cfg(test)]
mod tests {

    use crate::serial::{deserialize, serialize};
    use std::str::FromStr;

    #[test]
    pub fn test_serialize_btc_address() -> super::BtcResult<()> {
        let btc_addr =
            bitcoin::Address::from_str(&String::from("mxVFsFW5N4mu1HPkxPttorvocvzeZ7KZyk"))?;

        let btc_ser = serialize(&btc_addr);
        let btc_dser = deserialize(&btc_ser)?;

        assert_eq!(btc_addr, btc_dser);

        Ok(())
    }
}
