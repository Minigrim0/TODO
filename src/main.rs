pub mod models;
pub mod schema;
pub mod utils;
pub mod database;

use clap::Parser;
use crate::utils::{parse_args, Cli};


fn main() {
    let args = Cli::parse();
    println!("{:?}", args);

    parse_args(args);
}
