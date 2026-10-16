PreTxSim - Solana Privacy Hack (Private Payments / Privacy Tooling)

**Adapter:** solana -> SolanaAdapter
**Hackathon:** Solana Privacy Hack
**Track:** Private Payments __TRACK__ Privacy Tooling
**Repo:** Skywalkingzulu1/pretxsim_solana

PreTxSim is an air-gapped local EVM simulation proxy (Rust, revm). This
repository contains the single solana adapter, vendored with the shared
Adapter core, so it builds and tests as an independent crate.

## What it does

PreTxSim - Solana Privacy Hack (Private Payments / Privacy Tooling) runs a pure structural inspection of a transaction payload and
returns ALLOW / WARN / BLOCK with reasons, without broadcasting or
exposing keys to the network.

## Build and test

cargo build
cargo test

## Integration

Wire the adapter into your own binary via AdapterRegistry:

use pretxsim_*::adapters::{Adapter, AdapterRegistry};
use pretxsim_*::solana::__TYPE;

let reg = AdapterRegistry::new().register(Arc::new(SolanaAdapter));
let result = reg.inspect("solana", &tx).expect("adapter registered");

See the parent preTxSim repo (Skywalkingzulu1/pretxsim_poc) for the
full --target/--ci/--sim-file CLI and the agent.md multi-agent
coordination board.

## License

Proof of concept. License TBD.
