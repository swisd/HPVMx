use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use crate::database::{Column, DataType, Entry};
use crate::interface::local_format_timestamp_string;
use crate::println;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ColumnConstraint {
    PrimaryKey,
    Unique,
    NotNull,
    Default(String),
    References(String, String), // Table, Column

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnDef {
    pub name: String,
    pub data_type: String, // SERIAL, VARCHAR, TIMESTAMPTZ, etc.
    pub constraints: Vec<ColumnConstraint>,
}

#[derive(Debug)]
pub enum Command {
    Get { table_id: i32, row_id: i64 },
    Set { table_id: i32, row_id: i64, value: i32 },
    Select { table_id: i32, column: String, value: i32 }, // SELECT * FROM 1 WHERE id = 100
    Insert { table_id: i32, row_id: i64, payload: Vec<Entry> },   // INSERT INTO 1 VALUES (100, 42)
    Update { table_id: i32, row_id: i64, payload: Vec<Entry> },   // UPDATE 1 SET val = 42 WHERE id = 100
    Delete { table_id: i32, row_id: i64 },               // DELETE FROM 1 WHERE id = 100
    CreateTable { table_id: i32, name: String, columns: Vec<Column> },         // CREATE TABLE 1 users
    DropTable { table_id: i32 },                         // DROP TABLE 1
    Explain { table_id: i32 },                           // EXPLAIN 1
    ShowTables,                                          // .tables (SQLite style)
    Commit,                                              // COMMIT
    Help,                                                // .help
    Exit,                                                // .exit
    Invalid(String),
    CreateComplexTable {
        if_not_exists: bool,
        name: String,
        columns: Vec<ColumnDef>,
    },
    CreateIndex {
        if_not_exists: bool,
        index_name: String,
        table_name: String,
        columns: Vec<String>,
    },
    AlterTableAdd {
        table_id: i32,
        column: Column,
    },
    AlterTableDrop {
        table_id: i32,
        column_name: String,
    },
    ShowSchema { table_id: i32 },
    Display {
        table_id: i32,
        start: i64,
        end: Option<i64>,
    },
}

impl Command {
    pub fn parse(input: &str) -> Self {
        // 1. Clean the input: Add spaces around ( ) and , so split_whitespace() sees them
        let clean_input = input.trim()
            //.replace("(", " ( ")
            //.replace(")", " ) ")
            .replace(",", " "); // Commas aren't needed for our logic, so treat as space
        let it1 = Self::tokenize(&*clean_input);
        let it2 = it1.iter();
        let tokens: Vec<&str> = it2.map(String::as_str).collect();

        if tokens.is_empty() { return Command::Invalid("No input".into()); }
        match tokens.as_slice() {
            ["GET", tid, rid] => Command::Get {
                table_id: tid.parse().unwrap_or(0),
                row_id: rid.parse().unwrap_or(0),
            },
            ["SET", tid, rid, val] => Command::Set {
                table_id: tid.parse().unwrap_or(0),
                row_id: rid.parse().unwrap_or(0),
                value: val.parse().unwrap_or(0),
            },
            // SELECT * FROM 1 WHERE id = 100  OR  SELECT * FROM 1 WHERE val = 42
            ["SELECT", "*", "FROM", tid, "WHERE", col, "=", val] => {
                Command::Select {
                    table_id: tid.parse().unwrap_or(0),
                    column: col.to_lowercase(),
                    value: val.parse().unwrap_or(0),
                }
            }
            // INSERT INTO 1 VALUES 100 42
            ["INSERT", "INTO", tid, "ROW", rid, "VALUES", vals @ ..] => {
                let mut payload = Vec::new();
                for v_str in vals {
                    if let Some(dt) = Command::parse_datatype(v_str) {
                        payload.push(Entry { data: dt });
                    }
                }
                Command::Insert {
                    table_id: tid.parse().unwrap_or(0),
                    row_id: rid.parse().unwrap_or(0),
                    payload,
                }
            }
            // UPDATE 1 SET val = 99 WHERE id = 100
            // ["UPDATE", tid, "SET", "val", "=", val, "WHERE", "id", "=", rid] => {
            //     Command::Update {
            //         table_id: tid.parse().unwrap_or(0),
            //         row_id: rid.parse().unwrap_or(0),
            //         payload: val.parse().unwrap_or(0),
            //     }
            // },
            ["UPDATE", tid, "ROW", rid, "VALUES", vals @ ..] => {
                let mut payload = Vec::new();
                for v_str in vals {
                    if let Some(dt) = Command::parse_datatype(v_str) {
                        payload.push(Entry { data: dt });
                    }
                }
                Command::Update {
                    table_id: tid.parse().unwrap_or(0),
                    row_id: rid.parse().unwrap_or(0),
                    payload,
                }
            }
            // DELETE FROM 1 WHERE id = 100
            ["DELETE", "FROM", tid, "WHERE", "id", "=", rid] => {
                Command::Delete {
                    table_id: tid.parse().unwrap_or(0),
                    row_id: rid.parse().unwrap_or(0),
                }
            }
            // CREATE TABLE 1 users
            ["CREATE", "TABLE", tid, name, "(", col_defs @ .., ")"] => {
                println!("CREATE TABLE {} {} ({:#?})", tid, name, col_defs);
                let mut columns = Vec::new();
                // Simple loop to parse: "id", "Int32", "name", "Text"
                for chunk in col_defs.chunks(2) {
                    if chunk.len() == 2 {
                        let col_name = chunk[0].replace(",", "");
                        let col_type = chunk[1].replace(")", "");
                        println!("ctype {}", col_type);
                        columns.push(Column {
                            name: col_name,
                            data_type: Command::parse_type_to_variant(&col_type),
                        });
                    }
                }
                Command::CreateTable {
                    name: name.to_string(),
                    table_id: tid.parse().unwrap_or(0),
                    columns,
                }
            }
            ["ALTER", "TABLE", tid, "ADD", col_name, col_type] => {
                Command::AlterTableAdd {
                    table_id: tid.parse().unwrap_or(0),
                    column: Column {
                        name: col_name.to_string(),
                        data_type: Command::parse_type_to_variant(col_type),
                    },
                }
            }
            ["ALTER", "TABLE", tid, "DROP", "COLUMN", col_name] => {
                Command::AlterTableDrop {
                    table_id: tid.parse().unwrap_or(0),
                    column_name: col_name.to_string(),
                }
            }
            // DROP TABLE 1
            ["DROP", "TABLE", tid] => {
                Command::DropTable { table_id: tid.parse().unwrap_or(0) }
            }
            // EXPLAIN 1
            ["EXPLAIN", tid] => {
                Command::Explain { table_id: tid.parse().unwrap_or(0) }
            }
            // SQLite-style Meta Commands
            [".tables"] | ["TABLES"] => Command::ShowTables,
            [".help"] | ["HELP"] => Command::Help,
            [".exit"] | ["EXIT"] | ["QUIT"] => Command::Exit,
            ["COMMIT"] => Command::Commit,
            // Inside Command::parse match tokens.as_slice()
            [".schema", tid] | ["SCHEMA", tid] => Command::ShowSchema {
                table_id: tid.parse().unwrap_or(0),
            },

            ["DISPLAY", "TABLE", tid, "FROM", range] => {
                let parts: Vec<&str> = range.split("..").collect();
                let start = parts.get(0).and_then(|s| s.parse().ok()).unwrap_or(0);
                let end = parts.get(1).and_then(|&s| if s == "*" { None } else { s.parse().ok() });

                Command::Display {
                    table_id: tid.parse().unwrap_or(0),
                    start,
                    end,
                }
            }

            _ => Command::Invalid(format!("(?) Unknown syntax near '{}'", tokens[0])),
        }
    }
    fn parse_value(input: &str) -> Option<DataType> {
        if input == "Null" { return Some(DataType::Null); }

        // Find the first '(' and last ')'
        let start = input.find('(')?;
        let end = input.rfind(')')?;
        let type_part = &input[..start];
        let val_part = &input[start + 1..end];

        match type_part {
            "Int32" => val_part.parse::<i32>().ok().map(DataType::Int32),
            "Int16" => val_part.parse::<i16>().ok().map(DataType::Int16),
            "Int8" => val_part.parse::<i8>().ok().map(DataType::Int8),
            "Float64" => val_part.parse::<f64>().ok().map(DataType::Float64),
            "Text" => {
                // Remove surrounding quotes if present: Text("hello") -> hello
                let clean = val_part.trim_matches('"');
                Some(DataType::Text(clean.to_string()))
            }
            "Bool" => val_part.parse::<bool>().ok().map(DataType::Bool),
            "Timestamp" => val_part.parse::<String>().ok().map(DataType::Timestamp),
            _ => None,
        }
    }
    fn parse_datatype(input: &str) -> Option<DataType> {
        if input.starts_with("Int32(") && input.ends_with(")") {
            let val = input[6..input.len() - 1].parse::<i32>().ok()?;
            Some(DataType::Int32(val))
        } else if input.starts_with("Int16(") && input.ends_with(")") {
            let val = input[6..input.len() - 1].parse::<i16>().ok()?;
            Some(DataType::Int16(val))
        } else if input.starts_with("Int8(") && input.ends_with(")") {
            let val = input[5..input.len() - 1].parse::<i8>().ok()?;
            Some(DataType::Int8(val))
        } else if input.starts_with("Text(\"") && input.ends_with("\")") {
            let val = &input[6..input.len() - 2];
            Some(DataType::Text(val.to_string()))
        } else if input.starts_with("Float64(") && input.ends_with(")") {
            let val = input[8..input.len() - 1].parse::<f64>().ok()?;
            Some(DataType::Float64(val))
        } else if input.starts_with("Bool(") && input.ends_with(")") {
            let val = input[5..input.len() - 1].parse::<bool>().ok()?;
            Some(DataType::Bool(val))
        } else if input.starts_with("Timestamp(") && input.ends_with(")") {
            let mut val = input[10..input.len() - 1].parse::<String>().ok()?;
            if val.len() < 1 {
                val = local_format_timestamp_string();
            }
            Some(DataType::Timestamp(val))
        } else if input == "Null" {
            Some(DataType::Null)
        } else {
            println!("Unknown datatype {}", input);
            None
        }
    }
    fn parse_type_to_variant(type_str: &str) -> DataType {
        match type_str.to_lowercase().as_str() {
            "int32" => DataType::Int32(0),
            "int16" => DataType::Int16(0),
            "int8" => DataType::Int8(0),
            "float64" => DataType::Float64(0.0),
            "text" => DataType::Text(String::new()),
            "blob" => DataType::Blob(Vec::new()),
            "bool" => DataType::Bool(false),
            "timestamp" => DataType::Timestamp(local_format_timestamp_string()),
            _ => DataType::Null,
        }
    }

    fn tokenize(input: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut current = String::new();
        let mut in_quotes = false;

        for c in input.chars() {
            match c {
                '"' => {
                    in_quotes = !in_quotes;
                    current.push(c);
                }
                ' ' if !in_quotes => {
                    if !current.is_empty() {
                        tokens.push(current.clone());
                        current.clear();
                    }
                }
                _ => current.push(c),
            }
        }
        if !current.is_empty() { tokens.push(current); }
        tokens
    }
}

