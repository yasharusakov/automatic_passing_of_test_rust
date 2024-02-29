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

use std::{collections::HashMap, time::Duration};
use thirtyfour::prelude::{WebDriverResult, WebDriver, By, WebElement};

const QUESTION_SELECTOR: &str = ".test-content-text-inner > p";
const ANSWER_OPTION_IMAGE_SELECTOR: &str = ".question-option-image";
const ANSWER_OPTION_TEXT_SELECTOR: &str = ".question-option-inner-content";
const MULTIQUIZ_SAVE_BUTTON_SELECTOR: &str = ".test-multiquiz-save-button";
const MULTIQUIZ_SAVE_LINE_SPAN_SELECTOR: &str = ".test-multiquiz-save-line span";
const CURRENT_QUESTION_SELECTOR: &str = ".currentActiveQuestion";
const POLLING_INTERVAL: Duration = Duration::from_millis(2500);

pub async fn fetch_question_and_answers(driver: &WebDriver) -> WebDriverResult<(String, Vec<String>, Vec<WebElement>)> {
    let is_displayed_image = driver.find(By::Css(ANSWER_OPTION_IMAGE_SELECTOR)).await.is_ok();

    let question = driver.find(By::Css(QUESTION_SELECTOR)).await?.text().await?;

    let elements = driver.find_all(By::Css(match is_displayed_image {
        true => ANSWER_OPTION_IMAGE_SELECTOR,
        false => ANSWER_OPTION_TEXT_SELECTOR,
    })).await?;

    let mut answers: Vec<String> = Vec::new();

    for element in &elements {
        if is_displayed_image {
            let src = element.attr("style").await?.unwrap();
            answers.push(src[src.find("https").unwrap()..src.rfind('"').unwrap()].to_string());
        } else {
            let paragraphs = element.find_all(By::Css("p")).await?;
            let mut answer_text = String::from("");

            for paragraph in &paragraphs {
                let text = paragraph.text().await?;
                answer_text.push_str(text.trim())
            }

            answers.push(answer_text)
        }
    }

    Ok((question, answers, elements))
}

pub async fn pass_the_test(driver: &WebDriver, source_answers: &(HashMap<String, Vec<String>>, HashMap<Vec<String>, Vec<String>>)) -> WebDriverResult<()> {
    let (data, duplicated_data) = source_answers;
    let (question, answers, elements) = fetch_question_and_answers(&driver).await?;

    let is_multiquiz = driver.find(By::Css(MULTIQUIZ_SAVE_LINE_SPAN_SELECTOR)).await.is_ok();

    if data.contains_key(&question) {
        for (index, answer) in answers.iter().enumerate() {
            if data[&question].contains(answer) {
                elements[index].click().await?;
            }
        }
    } else {
        'outer: for (original_answers, duplicated_answers) in duplicated_data {
    
            for original_answer in original_answers {
                if !answers.contains(original_answer) {
                    continue 'outer;
                }
            }
    
            for (i, answer) in answers.iter().enumerate() {
                for duplicated_answer in duplicated_answers {
                    if duplicated_answer == answer {
                        elements[i].click().await?;
                    }
                }
            }
        }
    }

    if is_multiquiz {
        let save_button = driver.find(By::Css(MULTIQUIZ_SAVE_BUTTON_SELECTOR)).await?;
        save_button.click().await?;
    }

    Ok(())
}

async fn check_current_question(driver: &WebDriver, source_answers: &(HashMap<String, Vec<String>>, HashMap<Vec<String>, Vec<String>>), current_question_number: &mut i32) -> WebDriverResult<()> {
    let current_question = driver.find(By::Css(CURRENT_QUESTION_SELECTOR)).await?.text().await?;

    let current_question: i32 = current_question.trim().parse().expect("Can't parse");

    if current_question != *current_question_number {
        *current_question_number = current_question;
        pass_the_test(&driver, &source_answers).await?;
    }

    Ok(())
}

pub async fn listen_current_question(driver: &WebDriver, source_answers: &(HashMap<String, Vec<String>>, HashMap<Vec<String>, Vec<String>>)) {
    let mut current_question_number = 1;
    loop {
        let _ = check_current_question(&driver, &source_answers, &mut current_question_number).await;
        tokio::time::sleep(POLLING_INTERVAL).await;
    }
}