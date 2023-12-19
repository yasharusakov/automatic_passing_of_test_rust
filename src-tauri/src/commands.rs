use crate::web_driver::*;
use webbrowser;

#[tauri::command]
pub async fn launch_web_driver(username: &str, code: &str, source_answers_url: &str) -> Result<(), ()> {
    let data = Data {
        username,
        code,
        source_answers_url,
    };

    match web_driver(&data).await {
        Ok(_) => println!("WebDriver successfully has been launched"),
        Err(err) => println!("Error in launch_web_driver {}", err)
    }

    Ok(())
}

#[tauri::command]
pub async fn author_page() {
    if webbrowser::open("https://github.com/yasharusakov/automatic_passing_of_test_rust").is_ok() {
        println!("Successfully redirected to https://github.com/yasharusakov/automatic_passing_of_test_rust")
    }
}