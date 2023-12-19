// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod preparing;
mod passing;
mod web_driver;
mod commands;

use commands::*;

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![launch_web_driver, author_page])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}