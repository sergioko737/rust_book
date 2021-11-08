substrate/client/network/src/service.rs:							"📪 Libp2p listener ({}) closed gracefully",
substrate/client/network/src/service.rs:							"📪 Libp2p listener ({}) closed: {}",
substrate/client/consensus/babe/src/lib.rs:	info!(target: "babe", "👶 Starting BABE Authorship worker");
substrate/client/consensus/babe/src/lib.rs:					 "👶 New epoch {} launching at block {} (block slot {} >= start slot {}).",
substrate/client/consensus/babe/src/lib.rs:					 "👶 Next epoch starts at slot {}",
substrate/client/finality-grandpa/src/aux_schema.rs:	info!(target: "afg", "👴 Loading GRANDPA authority set \
substrate/frame/staking/src/migrations.rs:		crate::log!(info, "👜 staking bags-list migration passes PRE migrate checks ✅",);
substrate/frame/staking/src/migrations.rs:				"👜 completed staking migration to Releases::V8_0_0 with {} voters migrated",
substrate/frame/staking/src/migrations.rs:		crate::log!(info, "👜 staking bags-list migration passes POST migrate checks ✅",);
substrate-matrix-faucet/src/server/actions.ts:      logger.info('💸 sending tokens');
substrate-matrix-faucet/src/server/actions.ts:      logger.info('💰 checking balance');
substrate/client/network/src/discovery.rs:				"🔍 Discovered new external address for our node: {}",
substrate/client/network/src/protocol/sync.rs:				info!("💔 New peer with known bad best block {} ({}).", best_hash, best_number);
substrate/client/network/src/protocol/sync.rs:					info!("💔 New peer with unknown genesis hash {} ({}).", best_hash, best_number);
substrate/client/network/src/protocol/sync.rs:			error!(target: "sync", "💔 Called on_block_justification with a bad peer ID");
substrate/client/network/src/protocol/sync.rs:						"💔 Invalid block justification provided by {}: requested: {:?} got: {:?}",
substrate/client/network/src/protocol/sync.rs:							warn!("💔 Sent block with bad justification to import");
substrate/client/network/src/protocol/sync.rs:							"💔 Peer sent block with incomplete header to import",
substrate/client/network/src/protocol/sync.rs:							"💔 Verification failed for block {:?} received from peer: {}, {:?}",
substrate/client/network/src/protocol/sync.rs:							"💔 Block {:?} received from peer {} has been blacklisted",
substrate/client/network/src/protocol/sync.rs:					warn!(target: "sync", "💔 Error importing block {:?}: {:?}", hash, e);
substrate/client/network/src/protocol/sync.rs:				"💔 Error cleaning up pending extra justification data requests: {:?}",
substrate/client/network/src/protocol/sync.rs:						"💔 Ignored genesis block (#0) announcement from {}: {}",
substrate/client/network/src/protocol/sync.rs:							"💔 Ignored block (#{} -- {}) announcement from {} because all validation slots are occupied.",
substrate/client/network/src/protocol/sync.rs:						"💔 Ignored block (#{} -- {}) announcement from {} because all validation slots for this peer are occupied.",
substrate/client/network/src/protocol/sync.rs:							"💔 Block announcement validation of block {:?} errored: {}",
substrate/client/network/src/protocol/sync.rs:					"💔 Block announcement validation from peer {} finished for that no slot was allocated!",
substrate/client/network/src/protocol/sync.rs:			error!(target: "sync", "💔 Called on_block_announce with a bad peer ID");
substrate/client/network/src/protocol/sync.rs:			warn!(target: "sync", "💔  Unable to restart sync. :{:?}", e);
substrate/client/finality-grandpa/src/import.rs:					"👴 Imported justification for block #{} that triggers \
substrate/frame/bags-list/src/lib.rs:			concat!("[{:?}] 👜", $patter), <frame_system::Pallet<T>>::block_number() $(, $values)*
substrate/client/finality-grandpa/src/environment.rs:					"👴 Applying GRANDPA set change to new set with {} authorities",
substrate/client/finality-grandpa/src/environment.rs:				afg_log!(initial_sync, "👴 Applying GRANDPA set change to new set {:?}", set_ref);
substrate-matrix-faucet/src/bot/index.ts:    logger.warn(`🏴‍☠️ Ignored request from an ignored account: ${sender}`);
change  2020-10-16 08:03:14  ✌️  version 2.0.0-47f7d3f2e-x86_64-linux-gnu🔒
substrate/client/tracing/proc-macro/src/lib.rs:/// 2020-10-16 08:03:14  📋 Chain specification: Local Testnet
substrate/client/tracing/proc-macro/src/lib.rs:/// 2020-10-16 08:03:14  🏷 Node name: nice-glove-1401
substrate/client/tracing/proc-macro/src/lib.rs:/// 2020-10-16 08:03:14  👤 Role: LIGHT
substrate/client/cli/src/runner.rs:	/// 2020-06-03 16:14:21 🏷 Node name: jolly-rod-7462
substrate/client/cli/src/runner.rs:	/// 2020-06-03 16:14:21 👤 Role: FULL
substrate/client/cli/src/runner.rs:	/// 2020-06-03 16:14:21 💾 Database: RocksDb at /tmp/c/chains/flamingfir7/db
substrate/client/cli/src/runner.rs:	info!("📋 Chain specification: {}", config.chain_spec.name());
substrate/client/cli/src/runner.rs:	info!("🏷 Node name: {}", config.network.node_name);
substrate/client/cli/src/runner.rs:	info!("👤 Role: {}", config.display_role());
substrate/client/cli/src/runner.rs:		"💾 Database: {} at {}",
substrate/frame/election-provider-multi-phase/src/helpers.rs:			concat!("[#{:?}] 🗳  ", $pattern), <frame_system::Pallet<T>>::block_number() $(, $values)*
.
