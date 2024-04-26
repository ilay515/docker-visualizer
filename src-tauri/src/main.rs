// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod docker;

#[tauri::command]
fn parse_docker_compose(file_path: &str) -> String {
    docker::parse_docker_compose(file_path)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![parse_docker_compose])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
