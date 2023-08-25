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

use std::collections::HashMap;
use thirtyfour::prelude::{WebDriverResult, WebDriver, By, ElementWaitable};

use crate::utils::ReadLines;

pub async fn fetch_source_answers(driver: &WebDriver, source_answers_url: &String) -> WebDriverResult<HashMap<String, Vec<String>>> {
    driver.goto(source_answers_url).await?;

    driver.find(By::Css(".homework-stats .content-block"))
        .await?
        .wait_until()
        .displayed()
        .await?;

    let source_elements = driver.find_all(By::Css(".homework-stats .content-block")).await?;

    let mut data: HashMap<String, Vec<String>> = HashMap::new();

    for elem in &source_elements {
        let question = elem.find(By::Css(".homework-stat-question-line p"))
            .await?
            .text()
            .await?;

        let is_displayed_image = match elem.find(By::Css(".homework-stat-question-line .homework-stat-options .homework-stat-option-line .correct img")).await {
            Ok(image) => image.is_displayed().await?,
            Err(_) => false
        };

        let answers_elements = elem.find_all(By::Css(format!(".homework-stat-question-line .homework-stat-options .homework-stat-option-line .correct {}", if is_displayed_image { "img" } else { "p" }).as_str())).await?;

        let mut answers: Vec<String> = Vec::new();

        for answer_element in &answers_elements {
            if is_displayed_image {
                let answer_text = answer_element.attr("src").await?.unwrap();
                answers.push(answer_text);
            } else {
                let answer_text = answer_element
                    .text()
                    .await?
                    .trim()
                    .to_string();

                answers.push(answer_text);
            }
        }

        data.insert(question, answers);
    }

    Ok(data)
}

pub async fn join_test(driver: &WebDriver, read_lines: &ReadLines) -> WebDriverResult<()> {
    driver.goto("https://naurok.com.ua/test/join").await?;
    driver.find(By::Id("joinform-gamecode")).await?.send_keys(&read_lines.code).await?;
    driver.find(By::Id("joinform-name")).await?.send_keys(&read_lines.username).await?;
    driver.find(By::ClassName("join-button-test")).await?.click().await?;

    Ok(())
}