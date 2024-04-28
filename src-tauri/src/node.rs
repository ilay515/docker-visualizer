use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Position {
    x: u32,
    y: u32,
}

impl Position {
    pub fn new(x: u32, y: u32) -> Position {
        Position{x, y}
    }
}

#[derive(Debug, Serialize)]
struct NodeData {
    label: String,
}
#[derive(Debug, Serialize)]
pub struct NodeStyle {
    backgroundColor: String,
    width: u32,
    height: u32
}

impl NodeStyle {
    pub fn new(backgroundColor: String, width: u32, height: u32) -> NodeStyle {
        NodeStyle {backgroundColor, width, height}
    }
}

#[derive(Debug, Serialize)]
pub struct Node {
    id: String,
    node_type: String,
    data: NodeData,
    style: NodeStyle,
    className: String,
    position: Position,
}

impl Node {
    pub fn new(id: &str, label: &str, position: Position, style: NodeStyle) -> Node {
        Node {
            id: id.to_string(),
            node_type: "default".to_string(),
            data: NodeData {
                label: label.to_string()
            },
            style,
            className: "light".to_string(),
            position
        }
    }
}
