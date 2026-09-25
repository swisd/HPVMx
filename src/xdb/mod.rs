use alloc::string::ToString;
use crate::database::Database;
use crate::filesystem::FileSystem;
use crate::manager::DbManager;
use crate::server::DbServer;
use crate::vdebug_autoprefix;

pub mod cache;
pub mod database;
pub mod interface;
pub mod manager;
pub mod cmd_parser;
pub mod server;


pub fn open_db(path: &str) -> DbServer {
    let db = Database::load(path).unwrap_or_else(|_| {FileSystem::create(path); Database::new().save(path); Database::load(path).unwrap()});
    vdebug_autoprefix!(0, "successfully loaded database {}", path);
    let manager = DbManager::new(db, path.to_string());
    DbServer { manager }
}