//! Substrate Node Template CLI library.
#![warn(missing_docs)]

mod chain_spec;
#[macro_use]
mod service;
mod benchmarking;
mod cli;
mod command;
mod rpc;

// sc_cli::Result包含的Error类型和thiserror有关
fn main() -> sc_cli::Result<()> {
	//! 入口函数就在这里。
	command::run()
}
