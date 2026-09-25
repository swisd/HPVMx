use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use bincode::error::EncodeError;
use crate::cache::DbCache;
use crate::database::{Column, DataType, Database, Entry, Row, RowHeader, Table};
use crate::println;

pub struct DbManager {
    pub db: Database,
    pub cache: DbCache,
    pub path: String,
}

impl DbManager {
    pub fn new(db: Database, path: String) -> Self {
        let mut cache = DbCache::new();
        // Pre-load all tables from the DB into the cache
        for table in db.tables.iter() {
            cache.load_table(table.clone());
        }

        DbManager { db, cache, path }
    }

    /// Fetches a row, checking the cache first
    pub fn get_row(&self, table_id: i32, row_id: i64) -> Option<&Row> {
        self.cache.entries.get(&table_id).map(|entry| {
            entry.table.data.get(&row_id)
        }).flatten()
    }

    /// Updates or Inserts a row and marks the table as DIRTY
    pub fn upsert_row(&mut self, table_id: i32, row_id: i64, row: Row) {
        self.cache.update_row(table_id, row_id, row);
    }

    /// The "Commit" - Syncs dirty tables back to the Database struct and saves to disk
    pub fn commit(&mut self, path: &str) -> Result<(), String> {
        // 1. Move data from cache to the main DB struct if is_dirty is true
        self.db.sync_from_cache(&mut self.cache);

        // 2. Persist the entire DB to the file system
        let _ = self.db.save(path).map_err(|e| { format!("Encode error: {}", e) });

        println!("Transaction committed to disk.");
        Ok(())
    }
    // Helper to register a brand new table into the system
    pub fn create_table(&mut self, table: Table) {
        self.cache.load_table(table);
        // This puts it in cache; sync_from_cache will move it to self.db later
    }
    pub fn delete_row(&mut self, table_id: i32, row_id: i64) -> bool {
        self.cache.delete_row(table_id, row_id)
    }
    pub fn drop_table(&mut self, table_id: i32) -> bool {
        // Remove from cache
        let existed = self.cache.entries.remove(&table_id).is_some();
        // Remove from the main DB vector
        self.db.tables.retain(|t| t.header.table_id != table_id);
        existed
    }

    pub fn explain(&self, table_id: i32) -> String {
        match self.cache.entries.get(&table_id) {
            Some(entry) => {
                format!(
                    "--- EXPLAIN TABLE {} ---\r\n\
                     Root Table ID: {}\r\n\
                     Rows in Memory: {}\r\n\
                     Dirty State: {}\r\n\
                     Storage Type: BTreeMap (In-Memory B-Tree)\r\n",
                    entry.table.header.name,
                    table_id,
                    entry.table.data.len(),
                    if entry.is_dirty { "DIRTY (Unsaved Changes)" } else { "CLEAN (Synced to Disk)" }
                )
            }
            None => "Table not found in cache.\r\n".to_string(),
        }
    }
    pub fn select_where(&self, table_id: i32, col: &str, target: i32) -> Vec<(i64, &Row)> {
        let mut results = Vec::new();

        if let Some(entry) = self.cache.entries.get(&table_id) {
            if col == "id" {
                // Point Lookup (Fast O(log n))
                if let Some(row) = entry.table.data.get(&(target as i64)) {
                    results.push((target as i64, row));
                }
            } else if col == "val" {
                // Full Table Scan (Slow O(n))
                for (rid, row) in &entry.table.data {
                    // Check first payload entry (our 'val' column)
                    if let Some(Entry { data: DataType::Int32(v) }) = row.payload.get(0) {
                        if *v == target {
                            results.push((*rid, row));
                        }
                    }
                }
            }
        }
        results
    }
    pub fn insert_with_type_check(&mut self, table_id: i32, row_id: i64, payload: Vec<Entry>) -> Result<(), String> {
        let entry = self.cache.entries.get_mut(&table_id).ok_or("Table not found")?;
        let mut column_types = Vec::new(); // e.g., ["Int32", "Text"]
        for ent in &entry.table.columns {
            column_types.push(ent.data_type.as_str().to_string());
        }

        // 1. Check Column Count
        if payload.len() != column_types.len() {
            return Err(format!("Count mismatch: expected {}", column_types.len()));
        }

        // 2. Check Each Type
        for (i, item) in payload.iter().enumerate() {
            let expected = &column_types[i];
            let actual = match item.data {
                DataType::Int32(_) => "Int32",
                DataType::Int16(_) => "Int16",
                DataType::Int8(_) => "Int8",
                DataType::Text(_) => "Text",
                DataType::Blob(_) => "Blob",
                DataType::Bool(_) => "Bool",
                DataType::Null => "Null",
                _ => "Other",
            };

            if expected != actual {
                return Err(format!("Type mismatch at index {}: expected {}, got {}", i, expected, actual));
            }
        }

        // 3. Success
        entry.table.data.insert(row_id, Row { header: RowHeader { id: row_id}, payload });
        // // Inside the INSERT logic in Manager
        // let row_id = if user_provided_id.is_none() {
        //     let id = entry.table.next_serial;
        //     entry.table.next_serial += 1;
        //     id
        // } else {
        //     user_provided_id.unwrap()
        // };
        entry.is_dirty = true;
        Ok(())
    }
    pub fn add_column(&mut self, table_id: i32, col_name: String, default_val: DataType) -> Result<(), String> {
        let entry = self.cache.entries.get_mut(&table_id).ok_or("Table not found")?;

        // 1. Add the new Column definition to the schema
        entry.table.columns.push(Column { name: col_name, data_type: default_val.clone() });

        // 2. Data Migration: Append the default value to every existing row
        for row in entry.table.data.values_mut() {
            row.payload.push(Entry { data: default_val.clone() });
        }

        entry.is_dirty = true;
        Ok(())
    }
    pub fn drop_column(&mut self, table_id: i32, col_name: &str) -> Result<(), String> {
        let entry = self.cache.entries.get_mut(&table_id).ok_or("Table not found")?;

        let index = entry.table.columns.iter().position(|c| c.name == col_name)
            .ok_or("Column not found")?;

        entry.table.columns.remove(index);

        // Update data: remove the specific index from every row's payload
        for row in entry.table.data.values_mut() {
            if index < row.payload.len() {
                row.payload.remove(index);
            }
        }

        entry.is_dirty = true;
        Ok(())
    }
}