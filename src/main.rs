use clap::Parser;

pub mod cli;
pub mod schema;
pub mod models;
pub mod database;
pub mod utils;

use crate::cli::Cli;
use utils::parse_args;


fn main() {
    parse_args(Cli::parse());
}
