/* This file is part of DarkFi (https://dark.fi)
 *
 * Copyright (C) 2020-2024 Dyne.org foundation
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::{collections::HashSet, sync::Arc};

use log::{error, info};
use rpc_blocks::subscribe_blocks;
use sled_overlay::sled;
use smol::{lock::Mutex, stream::StreamExt};
use structopt_toml::{serde::Deserialize, structopt::StructOpt, StructOptToml};
use url::Url;

use darkfi::{
    async_daemonize,
    blockchain::{Blockchain, BlockchainOverlay},
    cli_desc,
    rpc::{
        client::RpcClient,
        server::{listen_and_serve, RequestHandler},
    },
    system::{StoppableTask, StoppableTaskPtr},
    util::path::expand_path,
    validator::utils::deploy_native_contracts,
    Error, Result,
};

use crate::metrics_store::MetricsStore;

/// Crate errors
mod error;

/// JSON-RPC requests handler and methods
mod rpc;
mod rpc_blocks;
mod rpc_statistics;
mod rpc_transactions;

/// Database functionality related to blocks
mod blocks;

/// Database functionality related to transactions
mod transactions;

/// Database functionality related to statistics
mod statistics;

/// Test utilities used for unit and integration testing
mod test_utils;

/// Database store functionality related to metrics
mod metrics_store;

/// Database store functionality related to contract metadata
mod contract_meta_store;

const CONFIG_FILE: &str = "blockchain_explorer_config.toml";
const CONFIG_FILE_CONTENTS: &str = include_str!("../blockchain_explorer_config.toml");

#[derive(Clone, Debug, Deserialize, StructOpt, StructOptToml)]
#[serde(default)]
#[structopt(name = "blockchain-explorer", about = cli_desc!())]
struct Args {
    #[structopt(short, long)]
    /// Configuration file to use
    config: Option<String>,

    #[structopt(short, long, default_value = "tcp://127.0.0.1:14567")]
    /// JSON-RPC listen URL
    rpc_listen: Url,

    #[structopt(long, default_value = "~/.local/share/darkfi/blockchain-explorer/daemon.db")]
    /// Path to daemon database
    db_path: String,

    #[structopt(long)]
    /// Reset the database and start syncing from first block
    reset: bool,

    #[structopt(short, long, default_value = "tcp://127.0.0.1:8340")]
    /// darkfid JSON-RPC endpoint
    endpoint: Url,

    #[structopt(short, long)]
    /// Set log file to output into
    log: Option<String>,

    #[structopt(short, parse(from_occurrences))]
    /// Increase verbosity (-vvv supported)
    verbose: u8,
}

/// Represents the service layer for the Explorer application, bridging the RPC layer and the database.
/// It encapsulates explorer business logic and provides a unified interface for core functionalities,
/// providing a clear separation of concerns between RPC handling and data management layers.
///
/// Core functionalities include:
///
/// - Data Transformation: Converting database data into structured responses suitable for RPC callers.
/// - Blocks: Synchronization, retrieval, counting, and management.
/// - Metrics: Providing metric-related data over the life of the chain.
/// - Transactions: Synchronization, calculating gas data, retrieval, counting, and related block information.
pub struct ExplorerService {
    /// Explorer database instance
    db: ExplorerDb,
}

impl ExplorerService {
    /// Creates a new `ExplorerService` instance
    ///
    /// The function sets up a new explorer database using the given [`String`] `db_path`, deploying
    /// native contracts needed for calculating transaction gas data.
    async fn new(db_path: String) -> Result<Self> {
        // Initialize explorer database
        let db = ExplorerDb::new(db_path)?;

        // Deploy native contracts needed to calculated transaction gas data and commit changes
        let overlay = BlockchainOverlay::new(&db.blockchain)?;
        deploy_native_contracts(&overlay, 10).await?;
        overlay.lock().unwrap().overlay.lock().unwrap().apply()?;

        Ok(Self { db })
    }
}

/// Represents the explorer database backed by a `sled` database connection, responsible for maintaining
/// persistent state required for blockchain exploration. It serves as the core data layer for the Explorer application,
/// storing and managing blockchain data, metrics, and contract-related information.
pub struct ExplorerDb {
    /// The main `sled` database connection used for data storage and retrieval
    pub sled_db: sled::Db,
    /// Local copy of the Darkfi blockchain used for block synchronization and exploration
    pub blockchain: Blockchain,
    /// Store for tracking chain-related metrics
    pub metrics_store: MetricsStore,
}

impl ExplorerDb {
    /// Creates a new `ExplorerDb` instance
    pub fn new(db_path: String) -> Result<Self> {
        let db_path = expand_path(db_path.as_str())?;
        let sled_db = sled::open(&db_path)?;
        let blockchain = Blockchain::new(&sled_db)?;
        let metrics_store = MetricsStore::new(&sled_db)?;
        info!(target: "blockchain-explorer", "Initialized explorer database {}, block count: {}", db_path.display(), blockchain.len());
        Ok(Self { sled_db, blockchain, metrics_store })
    }
}

/// Defines a daemon structure responsible for handling incoming JSON-RPC requests and delegating them
/// to the backend layer for processing. It provides a JSON-RPC interface for managing operations related to
/// blocks, transactions, and metrics.
///
/// Upon startup, the daemon initializes a background task to handle incoming JSON-RPC requests.
/// This includes processing operations related to blocks, transactions, and metrics by
/// delegating them to the backend and returning appropriate RPC responses. Additionally, the daemon
/// synchronizes blocks from the `darkfid` daemon into the explorer database and subscribes
/// to new blocks, ensuring that the local database remains updated in real-time.
pub struct Explorerd {
    /// Explorer service instance
    pub service: ExplorerService,
    /// JSON-RPC connection tracker
    pub rpc_connections: Mutex<HashSet<StoppableTaskPtr>>,
    /// JSON-RPC client to execute requests to darkfid daemon
    pub rpc_client: RpcClient,
}

impl Explorerd {
    /// Creates a new `BlockchainExplorer` instance.
    async fn new(db_path: String, endpoint: Url, ex: Arc<smol::Executor<'static>>) -> Result<Self> {
        // Initialize rpc client
        let rpc_client = RpcClient::new(endpoint.clone(), ex).await?;
        info!(target: "blockchain-explorer", "Created rpc client: {:?}", endpoint);

        // Initialize explorer service
        let service = ExplorerService::new(db_path).await?;

        Ok(Self { rpc_connections: Mutex::new(HashSet::new()), rpc_client, service })
    }
}

async_daemonize!(realmain);
async fn realmain(args: Args, ex: Arc<smol::Executor<'static>>) -> Result<()> {
    info!(target: "blockchain-explorer", "Initializing DarkFi blockchain explorer node...");
    let explorer = Explorerd::new(args.db_path, args.endpoint.clone(), ex.clone()).await?;
    let explorer = Arc::new(explorer);
    info!(target: "blockchain-explorer", "Node initialized successfully!");

    // JSON-RPC server
    info!(target: "blockchain-explorer", "Starting JSON-RPC server");
    // Here we create a task variable so we can manually close the task later.
    let rpc_task = StoppableTask::new();
    let explorer_ = explorer.clone();
    rpc_task.clone().start(
        listen_and_serve(args.rpc_listen, explorer.clone(), None, ex.clone()),
        |res| async move {
            match res {
                Ok(()) | Err(Error::RpcServerStopped) => explorer_.stop_connections().await,
                Err(e) => error!(target: "blockchain-explorer", "Failed starting sync JSON-RPC server: {}", e),
            }
        },
        Error::RpcServerStopped,
        ex.clone(),
    );

    // Sync blocks
    info!(target: "blockchain-explorer", "Syncing blocks from darkfid...");
    if let Err(e) = explorer.sync_blocks(args.reset).await {
        let error_message = format!("Error syncing blocks: {:?}", e);
        error!(target: "blockchain-explorer", "{error_message}");
        return Err(Error::DatabaseError(error_message));
    }

    // Subscribe blocks
    info!(target: "blockchain-explorer", "Subscribing to new blocks...");
    let (subscriber_task, listener_task) =
        match subscribe_blocks(explorer.clone(), args.endpoint, ex.clone()).await {
            Ok(pair) => pair,
            Err(e) => {
                let error_message = format!("Error setting up blocks subscriber: {:?}", e);
                error!(target: "blockchain-explorer", "{error_message}");
                return Err(Error::DatabaseError(error_message));
            }
        };

    // Signal handling for graceful termination.
    let (signals_handler, signals_task) = SignalHandler::new(ex)?;
    signals_handler.wait_termination(signals_task).await?;
    info!(target: "blockchain-explorer", "Caught termination signal, cleaning up and exiting...");

    info!(target: "blockchain-explorer", "Stopping JSON-RPC server...");
    rpc_task.stop().await;

    info!(target: "blockchain-explorer", "Stopping darkfid listener...");
    listener_task.stop().await;

    info!(target: "blockchain-explorer", "Stopping darkfid subscriber...");
    subscriber_task.stop().await;

    info!(target: "blockchain-explorer", "Stopping JSON-RPC client...");
    explorer.rpc_client.stop().await;

    Ok(())
}
