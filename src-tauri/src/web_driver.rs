use std::{process::Command, time::Duration};
use thirtyfour::prelude::{WebDriverResult, DesiredCapabilities, WebDriver};
use tokio::time::sleep;
use crate::passing::*;
use crate::preparing::*;

pub struct Data<'a> {
    pub username: &'a str,
    pub code: &'a str,
    pub source_answers_url: &'a str,
}

pub async fn web_driver<'a >(data: &'a Data<'a>) -> WebDriverResult<()> {
    if cfg!(target_os = "linux") {
        Command::new("geckodriver")
            .args(["--port", "4444"])
            .spawn()
            .expect("Failed to execute process");

        sleep(Duration::from_secs(1)).await;
    }

    let caps = DesiredCapabilities::firefox();
    let driver = WebDriver::new("http://localhost:4444", caps).await?;

    let source_answers = fetch_source_answers(&driver, data.source_answers_url).await?;

    join_test(&driver, data).await?;
    sleep(Duration::from_secs(3)).await;

    pass_the_test(&driver, &source_answers).await?;
    listen_current_question(&driver, &source_answers).await;

    driver.quit().await?;

    Ok(())
}