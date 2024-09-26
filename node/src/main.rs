//! Substrate Node Subtensor CLI library.
#![warn(missing_docs)]

mod chain_spec;
#[macro_use]
mod service;
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
mod client;
mod cli;
mod command;
mod ethereum;
mod rpc;

fn main() -> sc_cli::Result<()> {
    command::run()
}
