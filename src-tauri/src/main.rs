/* 
    Copyright 2023 yasharusakov

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License. 
*/

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod preparing;
mod passing;
mod web_driver;

use web_driver::*;

#[tauri::command]
async fn launch_web_driver(username: String, code: String, source_answers_url: String) {
    let data = Data {
        username,
        code,
        source_answers_url,
    };

    match web_driver(&data).await {
        Ok(_) => println!("good"),
        Err(err) => println!("{}", err)
    }
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![launch_web_driver])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}