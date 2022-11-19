#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs;
use serde::{Serialize, Deserialize};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Serialize, Deserialize, Debug)]
struct Entry {
    name: String,
    size: u64,
}

fn list_dir_impl(dir: &str) -> Result<Vec<Entry>, std::io::Error> {
    let mut result = Vec::new();
    for dir_entry in fs::read_dir(dir)? {
        let dir_entry = dir_entry?;
        let metadata = dir_entry.metadata()?;
        let entry = Entry{
            name: dir_entry.path().display().to_string(),
            size: if metadata.is_dir() { 0 } else { metadata.len() }
        };
        result.push(entry);
    }
    Ok(result)
}

#[tauri::command]
fn list_dir(dir: &str) -> Vec<Entry> {
    list_dir_impl(dir).unwrap_or(Vec::new())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, list_dir])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
