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

use std::io::stdin;

fn create_read_line(message: &str) -> String {
    eprint!("{}", message);

    let mut input = String::new();

    match stdin().read_line(&mut input) {
        Ok(_) => input,
        Err(error) => {
            println!("{}", error);
            create_read_line(message)
        }
    }
}

pub struct ReadLines {
    pub username: String,
    pub code: String,
    pub source_answers_url: String
}

impl ReadLines {
    pub fn new() -> Self {
        Self {
            username: create_read_line("Enter username: "),
            code: create_read_line("Enter code: "),
            source_answers_url: create_read_line("Enter source answers url: ")
        }
    }
}