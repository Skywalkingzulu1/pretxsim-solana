//! PreTxSim adapter for Solana Privacy Hack - Private Payments & Privacy Tooling.
//!
//! Air-gapped structural inspection for the Solana Privacy Hack hackathon.
//! Exposes the SolanaAdapter adapter plus the shared Adapter core.

pub mod adapters;
pub use adapters::{Adapter, AdapterResult, AdapterVerdict};
pub use adapters::solana::SolanaAdapter;
