use std::collections::HashMap;

#[derive(Debug)]
pub struct Node {
    children: Vec<Node>,
    node_type: NodeType,
}

#[derive(Debug)]
pub enum NodeType {
    Text(String),
    Element(ElementData),
}

#[derive(Debug)]
pub struct ElementData {
    tag_name: String,
    attributes: AttrMap,
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