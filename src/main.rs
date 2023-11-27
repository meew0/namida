#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(clippy::missing_safety_doc)]
#![feature(c_variadic)]
#![feature(extern_types)]

use clap::{Parser, Subcommand};

extern crate libc;
pub mod client;
pub mod common;
pub mod datagram;
pub mod message;
pub mod server;
pub mod types;

// TODO: automatically generate these
pub const COMPILE_DATE: &str = "Nov 16 2023";
pub const COMPILE_TIME: &str = "21:24:18";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Client(client::Parameter),
    Server(server::Parameter),
}

pub fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Client(parameter) => {
            client::main::interactive(parameter);
        }
        Commands::Server(parameter) => {
            server::main::serve(parameter)?;
        }
    }

    Ok(())
}
