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

use crate::web_driver::*;
use webbrowser;

#[tauri::command]
pub async fn launch_web_driver(username: String, code: String, source_answers_url: String) {
    let data = Data {
        username,
        code,
        source_answers_url
    };

    match web_driver(&data).await {
        Ok(_) => println!("WebDriver successfully has been launched"),
        Err(err) => println!("Error in launch_web_driver {}", err)
    }
}

#[tauri::command]
pub async fn author_page() {
    if webbrowser::open("https://github.com/yasharusakov/automatic_passing_of_test_rust").is_ok() {
        println!("Successfully redirected to https://github.com/yasharusakov/automatic_passing_of_test_rust")
    }
}
