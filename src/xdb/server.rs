use alloc::collections::BTreeMap;
use alloc::{format, vec};
use alloc::borrow::ToOwned;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::convert::Infallible;
use core::ops::{ControlFlow, FromResidual, Try};
use core::ptr::addr_of_mut;
use crate::database::{DataType, Entry, Row, RowHeader, Table, TableHeader};
use crate::manager::DbManager;
use crate::cmd_parser::Command;
use crate::{eprintln, println};

pub struct DbServer {
    pub(crate) manager: DbManager,
}

struct BufReader;

struct TcpStream;

impl TcpStream {
    pub(crate) fn expect(&self, p0: &str) -> TcpStream {
        todo!()
    }
}

impl TcpStream {
    pub(crate) fn flush(&self) -> Result<(), ()> {
        todo!()
    }
}

impl TcpStream {
    pub(crate) fn write_all(&self, p0: &[u8]) -> Result<(), ()> {
        todo!()
    }
}

impl TcpStream {
    pub(crate) fn try_clone(&self) -> Self {
        todo!()
    }
}

impl BufReader {
    pub fn new(stream: TcpStream) -> Self {
        todo!()
    }
    pub fn read_until(&self, byte: u8, mut buffer: &Vec<u8>) -> Result<(), ()> {
        todo!()
    }
}

struct TcpListener;

impl TcpListener {
    pub(crate) fn incoming(&self) -> Vec<TcpStream> {
        todo!()
    }
}

#[allow(unstable_features)]
impl FromResidual for TcpListener {
    fn from_residual(residual: <Self as Try>::Residual) -> Self {
        todo!()
    }
}

#[allow(unstable_features)]
impl Try for TcpListener {
    type Output = ();
    type Residual = Result<Infallible, String>;

    fn from_output(output: Self::Output) -> Self {
        todo!()
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        todo!()
    }
}

impl TcpListener {
    pub fn bind(address: &str) -> TcpListener {
        todo!()
    }
}

impl DbServer {
    pub fn start(mut self, address: &str) -> Result<(), ()> {
        let listener = TcpListener::bind(address);
        println!("Server listening on {}", address);

        for stream in listener.incoming() {
            self.handle_client(stream);
        }
        Ok(())
    }

    fn handle_client(&mut self, mut stream: TcpStream) {
        let mut reader = BufReader::new(stream.try_clone().expect("Failed to clone stream"));
        let mut byte_buffer = Vec::new(); // Store raw bytes instead of a String

        let _ = stream.write_all(b"Connected to xDB\r\ndb > ");
        let _ = stream.flush();

        loop {
            byte_buffer.clear();
            // Read until the newline byte (10)
            match reader.read_until(b'\n', &mut byte_buffer) {
                Ok(..) => {
                    let _ = stream.write_all(self.parse_command(&byte_buffer, &mut Vec::new()).as_bytes());
                    let _ = stream.flush();
                }
                Err(..) => {}
            }
        }
    }

    pub fn parse_command(&mut self, mut byte_buffer: &Vec<u8>, mut output_buffer: &mut Vec<u8>) -> String
    {
        // 1. Filter out Telnet IAC commands (bytes >= 128)
        let filtered_bytes: Vec<u8> = byte_buffer
            .iter()
            .filter(|&&b| b < 128) // Only keep standard ASCII
            .cloned()
            .collect();

        // 2. Convert the clean bytes to a String
        let line = String::from_utf8_lossy(&filtered_bytes);
        let trimmed = line.trim();

        // Log it on the server side to verify it's clean
        println!("Server received clean: {}", trimmed);

        if trimmed.is_empty() {
            let bytes: Vec<u8> = b"db > ".to_owned().into_iter().collect();
            output_buffer.extend(bytes)
        }

        let cmd = Command::parse(trimmed);

        let response = match cmd {
            Command::Get { table_id, row_id } => {
                match self.manager.get_row(table_id, row_id) {
                    Some(row) => format!("FOUND: {:?}\r\n", row.payload),
                    None => "NOT_FOUND\r\n".to_string(),
                }
            }
            Command::Set { table_id, row_id, value } => {
                let row = Row {
                    header: RowHeader { id: row_id },
                    payload: vec![Entry { data: DataType::Int32(value) }]
                };
                self.manager.upsert_row(table_id, row_id, row);
                "OK\r\n".to_string()
            }
            Command::Select { table_id, column, value } => {
                let results = self.manager.select_where(table_id, &column, value);
                if results.is_empty() { "EMPTY SET\r\n".into() } else {
                    results.iter().map(|(id, r)| format!("ID {}: {:?}\r\n", id, r.payload)).collect::<String>()
                }
            },
            Command::Insert { table_id, row_id, payload } => {
                // 1. Get table schema from cache
                if let Some(entry) = self.manager.cache.entries.get_mut(&table_id) {
                    let mut schema = Vec::new(); // e.g., ["Int32", "Text"]
                    for entry in &entry.table.columns {
                        schema.push(entry.data_type.as_str());
                    }
                    println!("schema: {:?}", schema);
                    println!("payload: {:?}", payload);

                    // 2. Validate row length
                    if payload.len() != schema.len() {
                        format!("ERROR: Expected {} values, got {}\r\n", schema.len(), payload.len())
                    } else {
                        // 3. Validate each data type
                        let mut mismatch = None;
                        for (i, entry) in payload.iter().enumerate() {
                            if entry.data.as_str() != schema[i] {
                                mismatch = Some(format!("Type mismatch at index {}: expected {:?}, got {}", i, schema[i], entry.data.as_str()));
                                break;
                            }
                        }

                        if let Some(err) = mismatch {
                            format!("ERROR: {}\r\n", err)
                        } else {
                            self.manager.upsert_row(table_id, row_id, Row { header: RowHeader { id: row_id }, payload });
                            "OK\r\n".to_string()
                        }
                    }
                } else {
                    "ERROR: Table not found\r\n".to_string()
                }
            }
            Command::Delete { table_id, row_id } => {
                if self.manager.delete_row(table_id, row_id) { "DELETED\r\n".into() } else { "NOT FOUND\r\n".into() }
            },
            Command::Update { table_id, row_id, payload } => {
                // todo: need to create a placeholder value to keep previous value while overwriting others, such as "#"
                if let Some(entry) = self.manager.cache.entries.get_mut(&table_id) {
                    let mut schema = Vec::new(); // e.g., ["Int32", "Text"]
                    for entry in &entry.table.columns {
                        schema.push(entry.data_type.as_str())
                    }

                    // 2. Validate row length
                    if payload.len() != schema.len() {
                        format!("ERROR: Expected {} values, got {}\r\n", schema.len(), payload.len())
                    } else {
                        // 3. Validate each data type
                        let mut mismatch = None;
                        for (i, entry) in payload.iter().enumerate() {
                            if entry.data.as_str() != schema[i] {
                                mismatch = Some(format!("Type mismatch at index {}: expected {:?}, got {}", i, schema[i], entry.data.as_str()));
                                break;
                            }
                        }

                        if let Some(err) = mismatch {
                            format!("ERROR: {}\r\n", err)
                        } else {
                            self.manager.upsert_row(table_id, row_id, Row { header: RowHeader { id: row_id }, payload });
                            "OK\r\n".to_string()
                        }
                    }
                } else {
                    "ERROR: Table not found\r\n".to_string()
                }
            },
            Command::CreateTable { name, table_id, columns } => {
                let new_table = Table {
                    name: name.clone(),
                    columns,
                    data: BTreeMap::new(),
                    next_serial: 1,
                    header: TableHeader { name, table_id, row_count: 0 },
                };
                self.manager.create_table(new_table);
                "TABLE CREATED\r\n".to_string()
            },
            Command::AlterTableAdd { table_id, column } => {
                self.manager.add_column(table_id, column.name, column.data_type);
                "COLUMN ADDED\r\n".to_string()
            },
            Command::AlterTableDrop { table_id, column_name } => {
                self.manager.drop_column(table_id, &column_name).expect("TODO: panic message");
                "COLUMN DROPPED\r\n".to_string()
            },
            Command::DropTable { table_id } => {
                if self.manager.drop_table(table_id) { "DROPPED\r\n".into() } else { "ERROR: Table not found\r\n".into() }
            },
            Command::Explain { table_id } => self.manager.explain(table_id),
            Command::ShowTables => {
                let list: Vec<String> = self.manager.cache.entries.values()
                    .map(|e| format!("[{}] {}", e.table.header.table_id, e.table.header.name)).collect();
                format!("TABLES:\r\n{}\r\n", list.join("\r\n"))
            },
            Command::Commit => {
                let pth = self.manager.path.clone();
                self.manager.commit(pth.as_str()).map(|_| "COMMITTED\r\n".into()).unwrap_or("WRITE ERROR\r\n".into())
            },
            Command::Help => "Try: SELECT * FROM 1 WHERE id = 100\r\n".into(),
            Command::Exit => return "".to_string(),
            Command::Invalid(msg) => format!("ERROR: {}\r\n", msg),
            Command::ShowSchema { table_id } => {
                if let Some(entry) = self.manager.cache.entries.get(&table_id) {
                    let mut out = format!("Table: {} (ID: {})\r\n", entry.table.name, table_id);
                    out.push_str("Columns:\r\n");
                    for col in &entry.table.columns {
                        out.push_str(&format!("  - {}: {}\r\n", col.name, col.data_type.as_str()));
                    }
                    out
                } else {
                    "ERROR: Table not found\r\n".into()
                }
            },

            Command::Display { table_id, start, end } => {
                if let Some(entry) = self.manager.cache.entries.get(&table_id) {
                    let mut out = String::new();
                    let table = &entry.table;

                    // 1. Build Column Names Row
                    let names: Vec<String> = table.columns.iter()
                        .map(|c| format!("{:<15}", c.name)) // Left-aligned with 15 spaces
                        .collect();
                    out.push_str(&names.join(" | "));
                    out.push_str("\r\n");

                    // 2. Build Column Types Row
                    let types: Vec<String> = table.columns.iter()
                        .map(|c| format!("{:<15}", c.data_type.as_str()))
                        .collect();
                    out.push_str(&types.join(" | "));
                    out.push_str("\r\n");

                    // 3. Build Separator
                    let total_width = (15 * names.len()) + (3 * (names.len() - 1));
                    out.push_str(&"-".repeat(total_width));
                    out.push_str("\r\n");

                    // 4. Build Data Rows
                    let range_end = end.unwrap_or(i64::MAX);
                    let rows = table.data.range(start..=range_end);

                    for (_id, row) in rows {
                        let row_vals: Vec<String> = row.payload.iter()
                            .map(|e| {
                                // Extract the inner value as a string for display
                                let val_str = match &e.data {
                                    DataType::Int32(v) => v.to_string(),
                                    DataType::Int16(v) => v.to_string(),
                                    DataType::Int8(v) => v.to_string(),
                                    DataType::Text(v) => v.clone(),
                                    DataType::Bool(v) => v.to_string(),
                                    DataType::Null => "NULL".to_string(),
                                    DataType::Float64(v) => v.to_string(),
                                    DataType::Timestamp(v) => v.to_string(),
                                    _ => format!("{:?}", e.data),
                                };
                                format!("{:<15}", val_str)
                            })
                            .collect();
                        out.push_str(&row_vals.join(" | "));
                        out.push_str("\r\n");
                    }

                    if out.trim().is_empty() || table.data.is_empty() {
                        "Table is empty or no rows in range.\r\n".into()
                    } else {
                        out
                    }
                } else {
                    "ERROR: Table not found\r\n".into()
                }
            },
            _ => "?".into(),
        };

        let mut final_output = response.clone();
        final_output.push_str("db > ");

        output_buffer.extend(final_output.as_bytes());
        response
    }
}