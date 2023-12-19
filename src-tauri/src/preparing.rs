use std::collections::HashMap;
use thirtyfour::prelude::{WebDriverResult, WebDriver, By, ElementWaitable};
use thirtyfour::WebElement;

use crate::web_driver::Data;

pub async fn fetch_source_answers(driver: &WebDriver, source_answers_url: &str) -> WebDriverResult<HashMap<String, Vec<String>>> {
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

        let mut answers_images: Vec<WebElement> = vec![];
        let mut answers_paragraphs: Vec<String> = vec![];

        let classname_answers = ".homework-stat-question-line .homework-stat-options .homework-stat-option-line .correct";

        if is_displayed_image {
            answers_images = elem.find_all(By::Css(format!("{} img", classname_answers).as_str())).await?;
        } else {
            let elements = elem.find_all(By::Css(classname_answers)).await?;
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

pub async fn join_test<'a >(driver: &WebDriver, data: &'a Data<'a>) -> WebDriverResult<()> {
    driver.goto("https://naurok.com.ua/test/join").await?;
    driver.find(By::Id("joinform-gamecode")).await?.send_keys(data.code).await?;
    driver.find(By::Id("joinform-name")).await?.send_keys(data.username).await?;
    driver.find(By::ClassName("join-button-test")).await?.click().await?;

    Ok(())
}