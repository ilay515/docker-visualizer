// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use docker::DockerCompose;

mod docker;
mod graph;

#[tauri::command]
fn parse_docker_compose(file_path: &str) -> String {
    let docker_compose: DockerCompose = docker::parse_docker_compose(file_path).unwrap();
    graph::build_graph(docker_compose).unwrap()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![parse_docker_compose])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
