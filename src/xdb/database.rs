#![allow(dead_code)]
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use bincode::config::standard;
use bincode::error::{DecodeError, EncodeError};
use serde::{Serialize, Deserialize};
use serde_big_array::BigArray;
use crate::cache::DbCache;
use crate::interface::{file_create, file_open, file_write_append_bytes};
use crate::println;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileHeader {
    pub magic: [u8; 16], // e.g., "MY_DB_FORMAT_V1"
    pub page_size: u32,
    pub version: u32,
    #[serde(with = "BigArray")]
    pub reserved: [u8; 72], // Padding to reach 100 bytes
}

#[derive(Serialize, Deserialize, Debug)]
#[derive(Clone)]
pub struct Database {
    pub file_header: FileHeader,
    pub tables: Vec<Table>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum DataType {
    Null,
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Float64(f64),
    Text(String),
    Blob(Vec<u8>),
    Bool(bool),
    Timestamp(String),
}

impl DataType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DataType::Null => "Null",
            DataType::Int8(_) => "Int8",
            DataType::Int16(_) => "Int16",
            DataType::Int32(_) => "Int32",
            DataType::Float64(_) => "Float64",
            DataType::Text(_) => "Text",
            DataType::Blob(_) => "Blob",
            DataType::Bool(_) => "Bool",
            DataType::Timestamp(_) => "Timestamp",
        }
    }
}

pub type ROWID = i64;
pub type TABLEID = i32;


// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct Table {
//     pub header: TableHeader,
//     pub columns: Vec<(String, DataType)>,
//     pub data: BTreeMap<ROWID, Row>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct ColumnSchema {
//     pub name: String,
//     pub is_primary: bool,
//     pub is_unique: bool,
//     pub is_nullable: bool,
//     pub default_val: Option<String>,
//     pub references: Option<(String, String)>, // (TargetTable, TargetColumn)
// }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Column {
    pub name: String,
    pub data_type: DataType
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
    pub data: BTreeMap<ROWID, Row>,
    pub next_serial: i64, // For SERIAL / Auto-increment
    pub header: TableHeader,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Row {
    pub header: RowHeader,
    pub payload: Vec<Entry>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RowHeader {
    pub id: ROWID,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Entry {
    pub data: DataType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TableHeader {
    pub name: String,
    pub table_id: TABLEID,
    pub row_count: u32,
}


impl Table {
    // 1. Initialize a new table
    pub fn new(name: &str, id: i32, col_defs: Vec<(String, DataType)>) -> Self {
        let columns = col_defs.into_iter()
            .map(|(name, data_type)| Column { name, data_type })
            .collect();

        Self {
            name: name.to_string(),
            columns,
            data: BTreeMap::new(),
            next_serial: 1, // Initialize auto-increment
            header: TableHeader { name: name.to_string(), table_id: id, row_count: 0 },
        }
    }

    // 2. Insert a row into the BTreeMap
    pub fn insert(&mut self, row_id: ROWID, values: Vec<DataType>) {
        let entries = values.into_iter().map(|v| Entry { data: v }).collect();
        let row = Row {
            header: RowHeader {
                id: row_id,
            },
            payload: entries,
        };

        self.data.insert(row_id, row);
        self.header.row_count += 1;
    }

    // 3. Simple select by ROWID
    pub fn get_row(&self, row_id: ROWID) -> Option<&Row> {
        self.data.get(&row_id)
    }


}


impl Database {
    /// Creates a fresh database instance with a standard 100-byte header
    pub fn new() -> Self {
        // Define a unique 16-byte magic string to identify your file format
        let mut magic = [0u8; 16];
        let identifier = b"RUST_xDB_v1";
        magic[..identifier.len()].copy_from_slice(identifier);

        Database {
            file_header: FileHeader {
                magic,
                page_size: 4096,      // Standard SQLite page size
                version: 1,           // Your format version
                reserved: [0u8; 72],  // Padding to ensure header is exactly 100 bytes
            },
            tables: Vec::new(),
        }
    }

    /// Helper to add a table to the database
    pub fn add_table(&mut self, table: Table) {
        self.tables.push(table);
    }

    pub fn save(&self, path: &str) -> Result<(), EncodeError> {
        let mut file = file_create(path);
        let config = standard();

        // 1. Serialize the fixed-size header
        let header_bytes = bincode::serde::encode_to_vec(&self.file_header, config)?;
        let _ = file_write_append_bytes(path, &header_bytes);

        // 2. Serialize the rest of the database
        let data_bytes = bincode::serde::encode_to_vec(&self.tables, config)?;
        let _ = file_write_append_bytes(path, &data_bytes);

        Ok(())
    }

    pub fn load(path: &str) -> Result<Self, DecodeError> {
        let mut file = file_open(path);
        let config = standard();

        // Use bincode to deserialize directly from the file handle
        // This removes the need for split_at() and manual buffering
        let (file_header, header_len): (FileHeader, usize) =
            bincode::serde::decode_from_slice(&*file, config)?;
        let (tables, _): (Vec<Table>, usize) =
            bincode::serde::decode_from_slice(&file[header_len..], config)?;

        Ok(Database { file_header, tables })
    }
    /// Synchronize the cache back into the database struct
    pub fn sync_from_cache(&mut self, cache: &mut DbCache)
    {
        let mut count = 0;

        for cached_entry in cache.entries.values_mut() {
            if cached_entry.is_dirty {
                let table_id = cached_entry.table.header.table_id;

                // Find the index of the table in the main list
                if let Some(pos) = self.tables.iter().position(|t| t.header.table_id == table_id) {
                    // Update existing table
                    self.tables[pos] = cached_entry.table.clone();
                } else {
                    // It's a brand new table! Add it to the main list
                    self.tables.push(cached_entry.table.clone());
                }

                cached_entry.is_dirty = false;
                count += 1;
            }
        }
        println!("Synced {} dirty tables to main Database struct.", count);
    }
}