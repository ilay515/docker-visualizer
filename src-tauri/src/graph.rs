use crate::docker::{DockerCompose, Service};
use serde::Serialize;
#[path = "node.rs"]
mod node;

use node::Node;
use serde_yaml::to_string;

use self::node::{NodeStyle, Position};

#[derive(Debug, Serialize)]
struct Graph {
    nodes: Vec<Node>,
}

fn build_host_node() -> Node {
    Node::new(
        "Host",
        "Host",
        Position::new(0, 0),
        NodeStyle::new("rgba(0, 0, 0, 0.2)".to_string(), 300, 300),
    )
}

fn services_to_nodes(services: &mut Vec<Service>, padding: u32) -> Vec<Node> {
    let nodes = services
        .iter_mut()
        .enumerate()
        .map(|(index, service)| {
            return Node::new(
                &index.to_string(),
                &service.name,
                Position::new(100 * index as u32 + padding, padding),
                NodeStyle::new("rgba(0, 0, 255, 1)".to_string(), 100, 50),
            );
        })
        .collect::<Vec<Node>>();
    nodes
}

pub fn build_graph(docker_compose: DockerCompose) -> Result<String, Box<dyn std::error::Error>> {
    let mut services = docker_compose.get_services();
    let mut nodes = services_to_nodes(&mut services, 10);
    nodes.push(build_host_node());
    let graph = Graph { nodes };
    serde_json::to_string(&graph).map_err(|e| e.into())
}
