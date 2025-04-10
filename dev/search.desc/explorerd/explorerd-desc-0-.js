searchState.loadedDescShard("explorerd", 0, "Defines a daemon structure responsible for handling …\nAuxiliary structure used to keep track of signals\nConfiguration management across multiple networks …\nConfiguration file to use\nEstablishes a connection to the configured darkfid …\nJSON-RPC client to execute requests to darkfid daemon\nDarkfi blockchain node endpoint to sync with when not in …\nCrate errors\nA asynchronous executor used to create an RPC client when …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nSignals handle\nAuxiliary task to handle SIGINT for forceful process abort\nHandles an incoming JSON-RPC request by executing the …\nAuxiliary task to handle SIGHUP, SIGTERM, SIGINT and …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nSet log file to output to\nLogs a banner displaying the startup details of the DarkFi …\nExplorer network (localnet, testnet, mainnet)\nCreates a new <code>BlockchainExplorer</code> instance.\nDisable synchronization and connections to <code>darkfid</code>, …\nReset the database and start syncing from first block\nManages JSON-RPC interactions for the explorer\nJSON-RPC connection tracker\nCore logic for block synchronization, chain data access, …\nExplorer service instance\nSIGHUP publisher to retrieve new configuration,\nManages persistent storage for blockchain, contracts, …\nTermination signal channel receiver\nIncrease verbosity (-vvv supported)\nHandler waits for termination signal\nRepresents an explorer configuration\nStruct representing the configuration for an explorer …\nRepresents network configurations for localnet, testnet, …\nReturns the currently active network configuration.\nPath to the explorer’s database.\nEndpoint of the DarkFi node JSON-RPC server to sync with.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the network configuration for specified network.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nLocal network configuration\nMainnet network configuration\nCurrent active network\nSupported network configurations\nCreates a new configuration from a given file path. If the …\nPath to the configuration if read from a file\nJSON-RPC settings used to set up a server that the …\nTestnet network configuration\nCustom RPC errors available for blockchain explorer. These …\nReturns the argument unchanged.\nLogs and converts a database error into a [<code>DatabaseError</code>]. …\nCalls <code>U::from(self)</code>.\nConstructs a <code>JsonError</code> representing a server error using …\nHelper function to convert <code>ExplorerdRpcError</code> into error …\nA RPC client for interacting with a Darkfid JSON-RPC …\nRPC block related requests\nEstablishes a connection to the Darkfid node, storing the …\nChecks if there is an active connection to Darkfid.\nRPC handlers for contract-related perations\nReturns the argument unchanged.\nRetrieves a block from at a given height returning the …\nRetrieves the last confirmed block returning the block …\nCalls <code>U::from(self)</code>.\nAuxiliary function that logs RPC request failures by …\nCreates a new client with an inactive connection.\nSends a ping request to the client’s darkfid endpoint to …\nSends a request to the client’s Darkfid JSON-RPC …\nJSON-RPC client used to communicate with the Darkfid …\nRPC handlers for blockchain statistics and metrics\nCloses the connection with the connected darkfid, …\nRPC handlers for transaction data, lookups, and processing\nRepresents the service layer for the Explorer application, …\nAdds provided <code>ContractId</code> with corresponding …\nAdds source code for a specified <code>ContractId</code> from a …\nHandles core block-related functionality\nCalculates the gas data for a given transaction, returning …\nImplements functionality for smart contracts\nJSON-RPC client used to execute requests to Darkfi …\nExplorer database instance\nDeploys native contracts required for gas calculation and …\nReturns the argument unchanged.\nFetches the latest <code>BaseStatistics</code> from the explorer …\nFetch a block given its header hash from the database.\nFetch a block given its height from the database.\nProvides the total block count.\nFetches the <code>BlockInfo</code> associated with a given <code>HeaderHash</code>.\nFetch all known blocks from the database.\nFetch blocks within a specified range from the database.\nFetches the total contract count of all deployed contracts …\nFetches <code>ContractMetaData</code> for a given <code>ContractId</code>, returning …\nFetches the source code content for a specified <code>ContractId</code> …\nFetches a list of source code file paths for a given …\nRetrieves all contracts from the store excluding native …\nAuxiliary function that retrieves <code>ContractRecord</code>s filtered …\nFetch the last N blocks from the database.\nFetches the latest metrics from the explorer database, …\nFetches the latest metrics from the explorer database, …\nRetrieves all native contracts (DAO, Deployooor, and …\nFetches a transaction given its header hash.\nProvides the transaction count of all the transactions in …\nFetches all known transactions from the database.\nFetches all transactions from the database for the given …\nFetches the <code>BlockInfo</code> associated with a given transaction …\nInitializes the explorer service by deploying native …\nCalls <code>U::from(self)</code>.\nFetch the last block from the database.\nLoads <code>ContractMetaData</code> for deployed native contracts into …\nLoads native contract source code into the explorer …\nCreates a new <code>ExplorerService</code> instance.\nAdds the provided <code>BlockInfo</code> to the block explorer database.\nHandles blockchain reorganizations (reorgs) during the …\nResets blocks in the database by clearing all block …\nResets the explorer state to the specified height. If a …\nResets the [<code>ExplorerDb::blockchain::blocks</code>] and […\nResets transactions in the database by clearing …\nPowers metrics gathering and analytical capabilities\nManages synchronization with darkfid\nSynchronizes blocks between the explorer and a Darkfi …\nConverts a <code>ContractId</code> into a <code>ContractRecord</code>.\nConverts a <code>Transaction</code> and its associated block …\nManages transaction data processing\nStructure representing a block record.\nReturns the argument unchanged.\nHeader hash identifier of the block\nBlock height\nCalls <code>U::from(self)</code>.\nThe block’s nonce. This value changes arbitrarily with …\nPrevious block hash\nMerkle tree root of the transactions hashes contained in …\nBlock producer signature\nBlock creation timestamp\nAuxiliary function to convert a <code>BlockRecord</code> into a …\nBlock version\nRepresents a contract record embellished with details that …\nThe optional description of the contract\nReturns the argument unchanged.\nThe Contract ID as a string\nCalls <code>U::from(self)</code>.\nThe optional name of the contract\nAuxiliary function to convert a <code>ContractRecord</code> into a …\nAuxiliary function that extracts source code files from a …\nStructure representing basic statistic extracted from the …\nStructure representing metrics extracted from the database.\nCurrent blockchain epoch (based on current height)\nReturns the argument unchanged.\nReturns the argument unchanged.\nCurrent blockchain height\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nBlockchains’ last block hash\nMetrics used to store explorer statistics\nAuxiliary function to convert <code>BaseStatistics</code> into a …\nAuxiliary function to convert <code>MetricStatistics</code> into a …\nBlockchain total blocks\nBlockchain total transactions\nSubscribes to darkfid’s JSON-RPC notification endpoint …\nStructure representing a <code>TRANSACTIONS_TABLE</code> record.\nGas used for deployments\nReturns the argument unchanged.\nHeader hash identifier of the block this transaction was …\nCalls <code>U::from(self)</code>.\nTransaction payload\nGas used for creating the transaction signature\nTime transaction was added to the block\nAuxiliary function to convert a <code>TransactionRecord</code> into a …\nTotal gas used for processing transaction\nTransaction hash identifier\nGas used by WASM\nGas used by ZK circuit operations\nRepresents the explorer database backed by a <code>sled</code> database …\nLocal copy of the Darkfi blockchain used for block …\nStore for managing contract metadata, source code, and …\nStores, manages, and provides access to contract metadata\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nStores, manages, and provides access to explorer metrics\nStore for tracking chain-related metrics\nCreates a new <code>ExplorerDb</code> instance\nThe main <code>sled</code> database connection used for data storage …\nRepresents contract metadata containing additional …\nThe <code>ContractMetadataStoreOverlay</code> provides write operations …\nRepresents a source file containing its file path as a …\nContract metadata tree name.\nContract source code tree name.\nDeletes source code associated with provided <code>ContractId</code> …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nRetrieves associated contract metadata for a given …\nRetrieves a source content as a <code>String</code> given a <code>ContractId</code> …\nRetrieves all the source file paths associated for …\nAdds contract metadata using provided <code>ContractId</code> and …\nInserts <code>ContractId</code> and <code>ContractMetaData</code> pairs into the …\nAdds contract source code <code>ContractId</code> and <code>Vec</code> of …\nInserts <code>ContractSourceFile</code>s associated with provided …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nChecks if there is contract metadata stored.\nProvides the number of stored <code>ContractMetaData</code>.\nAcquires a lock on the database, opening a specified tree …\nPrimary sled tree for storing contract metadata, utilizing …\nCreates a <code>ContractMetaStore</code> instance.\nInstantiate a <code>ContractMetadataStoreOverlay</code> over the …\nCreates a <code>ContractSourceFile</code> instance.\nPointer to the overlay used for accessing and performing …\nPointer to the underlying sled database used by the store …\nSled tree for storing contract source code, utilizing …\nThe time interval for <code>GasMetricsKey</code>s in the main tree, …\nStructure for managing gas metrics across all transactions …\nRepresents a key used to store and fetch metrics in the …\nProvides a unified method for creating new instances of …\nRepresents metrics used to capture key statistical data.\nThe <code>MetricStore</code> serves as the entry point for managing …\nThe <code>MetricsStoreOverlay</code> provides write operations for …\nGas metrics <code>by_height</code> tree that contains all metrics by …\nGas metrics tree name.\nTransaction gas data tree name.\nAdds new <code>GasData</code> to the existing accumulated values.\nProvides the average of the gas used for deployments.\nProvides the average of the gas used to sign transactions.\nProvides the average of the total gas used.\nProvides the average of the gas used across WASM …\nProvides the average of the gas consumed across …\nSled tree for storing gas metrics by height, utilizing …\nChecks if provided <code>GasMetricsKey</code> exists in the store’s …\nPointer managed by the <code>MetricsStore</code> that references the …\nGas consumed for deployments across transactions.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nConverts a <code>sled</code> key into a <code>GasMetricsKey</code> by deserializing …\nFetches <code>GasMetrics</code>s associated with the provided slice of …\nFetches all <code>GasMetrics</code> from the main tree without …\nFetches <code>GasMetrics</code>s associated with the provided slice of …\nFetches the most recent gas metrics from …\nFetches the most recent <code>GasMetrics</code> and its associated …\nFetches the most recent gas metrics from …\nFetches the most recent <code>GasMetrics</code> and its associated …\nFetches the <code>GasData</code> associated with the provided …\nGets the inner <code>DateTime</code> value.\nInserts <code>GasMetricsKey</code> and <code>GasMetrics</code> pairs into the store…\nInserts provided <code>u32</code> height and <code>GasMetrics</code> pairs into the …\nAdds the provided <code>TransactionHash</code> and <code>GasData</code> pairs to the …\nAdds gas metrics for a specific block of transactions to …\nInserts <code>TransactionHash</code> and <code>GasData</code> pairs into the store’…\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nChecks if there are any gas metrics stored.\nChecks if transaction gas data metrics are stored.\nProvides the number of stored metrics in the main tree.\nProvides the number of stored metrics by height.\nReturns the number of transaction gas usage metrics stored.\nPrimary sled tree for storing gas metrics, utilizing …\nThe largest value in the series of measured metrics.\nThe smallest value in the series of measured metrics.\nInstantiate a <code>MetricsStoreOverlay</code> over the provided […\nConstructs a <code>Metrics</code> instance with provided parameters.\nCreates a <code>GasMetrics</code> instance.\nCreates a <code>MetricsStore</code> instance by opening the necessary …\nCreates a new <code>GasMetricsKey</code> from a source that implements …\nNormalizes the given <code>DateTime</code> to the start of hour.\nNormalizes a given <code>Timestamp</code> to the start of the hour.\nPointer to the overlay used for accessing and performing …\nResets gas metrics in the <code>SLED_GAS_METRICS_TREE</code> and …\nResets the gas metrics in the store to a specified <code>height</code> …\nReverts gas metric entries from …\nThis function reverts gas metric entries in the …\nGas used metrics related to signatures across transactions.\nPointer to the underlying sled database used by the store …\nAn aggregate value that represents the sum of the metrics.\nThe time the metrics was calculated\nConverts the <code>GasMetricsKey</code> into a key suitable for use …\nOverall gas consumed metrics across all transactions.\nSled tree for storing transaction gas data, utilizing …\nRepresents the total count of transactions tracked by the …\nGas used across all executed wasm transactions.\nGas consumed across all zk circuit computations.")