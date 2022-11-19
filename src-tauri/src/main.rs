#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs;
use serde::{Serialize, Deserialize};
use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayEvent, Manager};

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
        .setup(|app| {
            let tray_handle = SystemTray::new()
              .with_menu(
                SystemTrayMenu::new()
                    .add_item(CustomMenuItem::new("flip", "Flip"))
                    .add_item(CustomMenuItem::new("quit", "Quit"))
              )
              .build(app)?;
            Ok(())
          })
        .on_system_tray_event(|app, event| {
            let main_window = app.get_window("main").unwrap();
            match event {
                SystemTrayEvent::MenuItemClick { id, .. } => {
                    match id.as_str() {
                        "quit" => app.exit(0),
                        "flip" => if main_window.is_visible().unwrap() {
                                main_window.hide().unwrap()
                            } else {
                                main_window.show().unwrap()
                            },
                        _ => {
                        }
                    }

                }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![greet, list_dir])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
