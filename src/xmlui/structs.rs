use alloc::string::String;
use alloc::vec::Vec;
use hashbrown::HashMap;

pub type NodeProp = (String, String);

enum NodeDataType {
    Text(String),
    Number(i64),
    Boolean(bool),
}

pub struct Node {
    pub children: Option<Vec<Node>>,
    pub properties: HashMap<String, String>,
    pub data: NodeDataType,
}