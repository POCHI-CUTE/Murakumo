use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Node {
    pub children: Vec<Node>,
    pub node_type: NodeType,
}

#[derive(Debug)]
pub enum NodeType {
    Text(String),
    Element(ElementData),
}

#[derive(Debug)]
pub struct ElementData {
    pub tag_name: String,
    pub attributes: AttrMap,
}

pub type AttrMap = HashMap<String, String>;

pub fn text(data: String) -> Node {
    return Node {
        children: Vec::new(),
        node_type: NodeType::Text(data),
    };
}

pub fn elem(tag_name: String, attributes: AttrMap, children: Vec<Node>) -> Node {
    return Node {
        children,
        node_type: NodeType::Element(ElementData {
            tag_name,
            attributes,
        }),
    };
}

impl ElementData {
    pub fn id(&self) -> Option<&String> {
        self.attributes.get("id")
    }

    pub fn classes(&self) -> HashSet<&str> {
        match self.attributes.get("class") {
            Some(classlist) => classlist.split(' ').collect(),
            None => HashSet::new(),
        }
    }
}
