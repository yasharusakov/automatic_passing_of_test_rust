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

use std::{process::Command, time::Duration};
use thirtyfour::prelude::{WebDriverResult, DesiredCapabilities, WebDriver};
use tokio::time::sleep;
use crate::passing::*;
use crate::preparing::*;

pub struct Data {
    pub username: String,
    pub code: String,
    pub source_answers_url: String
}

pub async fn web_driver(data: &Data) -> WebDriverResult<()> {
    if cfg!(target_os = "linux") {
        Command::new("geckodriver")
            .args(["--port", "4444"])
            .spawn()
            .expect("Failed to execute process");

        sleep(Duration::from_secs(1)).await;
    }

    let caps = DesiredCapabilities::firefox();
    let driver = WebDriver::new("http://localhost:4444", caps).await?;

    let source_answers = fetch_source_answers(&driver, &data.source_answers_url).await?;

    join_test(&driver, &data).await?;
    sleep(Duration::from_secs(3)).await;

    pass_the_test(&driver, &source_answers).await?;
    listen_current_question(&driver, &source_answers).await;

    driver.quit().await?;

    Ok(())
}
