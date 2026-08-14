use alloc::string::String;
use alloc::vec::Vec;
use hashbrown::HashMap;

pub type NodeProp = (String, String);

pub struct Node {
    pub children: Option<Vec<Node>>,
    pub properties: HashMap<String, String>,
}