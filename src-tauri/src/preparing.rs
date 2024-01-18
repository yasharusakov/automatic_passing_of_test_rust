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
use thirtyfour::WebElement;

use crate::web_driver::Data;

const ANSWERS_SELECTOR: &str = ".homework-stat-question-line .homework-stat-options .homework-stat-option-line .correct";
const SOURCE_ELEMENTS_SELECTOR: &str = ".homework-stats .content-block";

pub async fn fetch_source_answers(driver: &WebDriver, source_answers_url: &String) -> WebDriverResult<HashMap<String, Vec<String>>> {
    driver.goto(source_answers_url).await?;

    driver.find(By::Css(SOURCE_ELEMENTS_SELECTOR))
        .await?
        .wait_until()
        .displayed()
        .await?;

    let source_elements = driver.find_all(By::Css(SOURCE_ELEMENTS_SELECTOR)).await?;

    let mut data: HashMap<String, Vec<String>> = HashMap::new();

    for elem in &source_elements {
        let question = elem.find(By::Css(".homework-stat-question-line p")).await?.text().await?;

        let is_displayed_image = elem.find(By::Css(format!("{} img", ANSWERS_SELECTOR).as_str())).await.is_ok();

        let mut answers_images: Vec<WebElement> = vec![];
        let mut answers_paragraphs: Vec<String> = vec![];

        if is_displayed_image {
            answers_images = elem.find_all(By::Css(format!("{} img", ANSWERS_SELECTOR).as_str())).await?;
        } else {
            let elements = elem.find_all(By::Css(ANSWERS_SELECTOR)).await?;
            for element in &elements {
                let mut item = String::from("");
                let paragraphs = element.find_all(By::Css("p")).await?;
                for p in &paragraphs {
                    let text = p
                        .text()
                        .await?
                        .trim()
                        .to_string();

                    item.push_str(&text)
                }
                answers_paragraphs.push(item.trim().to_string())
            }
        }

        let mut answers: Vec<String> = Vec::new();

        if is_displayed_image {
            for answer_element in answers_images {
                let answer_text = answer_element.attr("src").await?.unwrap();
                answers.push(answer_text);
            }
        } else {
            answers = answers_paragraphs;
        }

        data.insert(question, answers);
    }

    Ok(data)
}

pub async fn join_test(driver: &WebDriver, data: &Data) -> WebDriverResult<()> {
    driver.goto("https://naurok.com.ua/test/join").await?;
    driver.find(By::Id("joinform-gamecode")).await?.send_keys(&data.code).await?;
    driver.find(By::Id("joinform-name")).await?.send_keys(&data.username).await?;
    driver.find(By::ClassName("join-button-test")).await?.click().await?;

    Ok(())
}