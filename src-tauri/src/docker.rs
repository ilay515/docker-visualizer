use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Deserialize, Serialize)]
pub struct Service {
    pub name: String,
    pub image: String,
    pub ports: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ServiceData {
    image: String,
    ports: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct DockerCompose {
    services: HashMap<String, ServiceData>,
}

impl DockerCompose {
    pub fn get_services(&self) -> Vec<Service> {
        let services: Vec<Service> = self
            .services
            .iter()
            .map(|(name, service_data)| Service {
                name: name.clone(),
                image: service_data.image.clone(),
                ports: service_data.ports.clone(),
            })
            .collect();
        services
    }
}

pub fn parse_docker_compose(file_path: &str) -> Result<DockerCompose, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let docker_compose: DockerCompose = serde_yaml::from_reader(reader)?;
    Ok(docker_compose)
}
