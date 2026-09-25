use crate::database::{Row, Table};
use crate::println;
use alloc::vec::Vec;
use hashbrown::HashMap;

pub struct CachedTable {
    pub table: Table,
    pub is_dirty: bool, // The "Dirty Bit"
}

pub struct DbCache {
    // Maps TableID to our tracked CachedTable
    pub entries: HashMap<i32, CachedTable>,
}

impl DbCache {
    pub fn new() -> Self {
        Self { entries: HashMap::new() }
    }

    /// Load a table into cache (initially clean)
    pub fn load_table(&mut self, table: Table) {
        let id = table.header.table_id;
        self.entries.insert(id, CachedTable {
            table,
            is_dirty: false,
        });
    }

    /// Mark a table as dirty when we modify it
    pub fn update_row(&mut self, table_id: i32, row_id: i64, new_row: Row) {
        if let Some(entry) = self.entries.get_mut(&table_id) {
            entry.table.data.insert(row_id, new_row);
            entry.is_dirty = true; // Flag it!
            println!("Table {} is now DIRTY", table_id);
        }
    }
    pub fn delete_row(&mut self, table_id: i32, row_id: i64) -> bool {
        if let Some(entry) = self.entries.get_mut(&table_id) {
            // .remove() returns the value if it existed
            if entry.table.data.remove(&row_id).is_some() {
                entry.is_dirty = true;
                return true;
            }
        }
        false
    }
}

pub struct LruCache {
    pub capacity: usize,
    pub storage: HashMap<i32, Table>,
    pub order: Vec<i32>, // Tracks which IDs were accessed recently
}

impl LruCache {
    pub fn insert(&mut self, id: i32, table: Table) {
        if self.storage.len() >= self.capacity {
            let oldest = self.order.remove(0);
            self.storage.remove(&oldest);
        }
        self.storage.insert(id, table);
        self.order.push(id);
    }
}