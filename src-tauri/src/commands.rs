use crate::web_driver::*;
use webbrowser;

#[tauri::command]
pub async fn launch_web_driver(username: String, code: String, source_answers_url: String) {
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

#[tauri::command]
pub async fn author_page() {
    if webbrowser::open("https://github.com/yasharusakov/automatic_passing_of_test_rust").is_ok() {
        println!("Successful redirected to https://github.com/yasharusakov/automatic_passing_of_test_rust")
    }
}

#[tauri::command]
pub async fn rust_link() {
    if webbrowser::open("https://www.rust-lang.org").is_ok() {
        println!("Successful redirected to https://www.rust-lang.org")
    }
}