mod tests;

pub mod schema;
pub mod models;
pub mod database;
pub mod display;
pub mod tasks;
pub mod validation;
pub mod utils;

pub fn init() {
    utils::ensure_db_path();
}
