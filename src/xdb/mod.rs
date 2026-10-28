use alloc::string::ToString;
use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool, Ordering};
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

static mut GLOBAL_DB: Option<UnsafeCell<DbServer>> = None;
static GLOBAL_DB_READY: AtomicBool = AtomicBool::new(false);

pub fn init_global_db(path: &str) {
    if GLOBAL_DB_READY.load(Ordering::SeqCst) {
        return;
    }
    let db_server = open_db(path);
    unsafe {
        GLOBAL_DB = Some(UnsafeCell::new(db_server));
    }
    GLOBAL_DB_READY.store(true, Ordering::SeqCst);
}

pub fn get_global_db_mut() -> Option<&'static mut DbServer> {
    if !GLOBAL_DB_READY.load(Ordering::SeqCst) {
        return None;
    }
    unsafe {
        GLOBAL_DB.as_ref().map(|cell| &mut *cell.get())
    }
}

pub fn open_db(path: &str) -> DbServer {
    let db = Database::load(path).unwrap_or_else(|_| {FileSystem::create(path); Database::new().save(path); Database::load(path).unwrap()});
    vdebug_autoprefix!(0, "successfully loaded database {}", path);
    let manager = DbManager::new(db, path.to_string());
    DbServer { manager }
}