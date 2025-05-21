use clap::Parser;

mod tests;

pub mod cli;
pub mod database;
pub mod display;
pub mod models;
pub mod parser;
pub mod schema;
pub mod tasks;
pub mod utils;
pub mod validation;

use crate::cli::Cli;
use parser::parse_args;

fn main() {
    utils::ensure_db_path();
    parse_args(Cli::parse());
}
