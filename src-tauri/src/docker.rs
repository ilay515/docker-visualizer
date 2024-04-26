use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Serialize, Deserialize)]
struct Service {
    image: String,
    ports: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DockerCompose {
    services: std::collections::HashMap<String, Service>,
}

pub fn parse_docker_compose(file_path: &str) -> String {
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);
    let docker_compose_yaml: DockerCompose =
        serde_yaml::from_reader(reader).expect("Failed to parse YAML");
    serde_json::to_string(&docker_compose_yaml).expect("Failed to convert to JSON")
}
