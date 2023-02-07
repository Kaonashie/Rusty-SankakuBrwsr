#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn create_dir(path: &str) -> String {
    if std::path::Path::new(&path).is_dir() {
        format!("Directory already exists...")
    } else {
        std::fs::create_dir(path).expect("Failed to create directory.");
        format!("Created directory: {}", path)
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, create_dir])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
