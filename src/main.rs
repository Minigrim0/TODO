use clap::Parser;

mod tests;

pub mod cli;
pub mod schema;
pub mod models;
pub mod database;
pub mod display;
pub mod tasks;
pub mod validation;
pub mod utils;
pub mod parser;

use crate::cli::Cli;
use parser::parse_args;

fn main() {
    utils::ensure_db_path();
    parse_args(Cli::parse());
}
