/*
    Copyright © 2021 trickybestia <trickybestia@gmail.com>

    This file is part of linux-discord-rich-presence.

    linux-discord-rich-presence is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    linux-discord-rich-presence is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with linux-discord-rich-presence.  If not, see <https://www.gnu.org/licenses/>.
*/

use crate::shell::Shell;
use std::{path::Path, str::FromStr};

pub struct ConfigShell {
    shell: Shell,
}

impl ConfigShell {
    pub async fn new(config_path: &Path) -> Self {
        Self {
            shell: Shell::new(config_path).await,
        }
    }

    async fn execute_function(&mut self, function_name: &str) -> Option<String> {
        let output = self.shell.execute(function_name).await;

        if output.is_empty() {
            None
        } else {
            Some(output)
        }
    }

    async fn execute_function_and_parse<T: FromStr>(&mut self, function_name: &str) -> Option<T> {
        let output = self.shell.execute(function_name).await;

        if let Ok(value) = output.parse() {
            Some(value)
        } else {
            None
        }
    }

    pub async fn application_id(&mut self) -> Option<u64> {
        self.execute_function_and_parse("application_id").await
    }

    pub async fn update_delay(&mut self) -> Option<u64> {
        self.execute_function_and_parse("update_delay").await
    }

    pub async fn state(&mut self) -> Option<String> {
        self.execute_function("state").await
    }

    pub async fn details(&mut self) -> Option<String> {
        self.execute_function("details").await
    }

    pub async fn large_image_key(&mut self) -> Option<String> {
        self.execute_function("large_image_key").await
    }

    pub async fn large_image_text(&mut self) -> Option<String> {
        self.execute_function("large_image_text").await
    }

    pub async fn small_image_key(&mut self) -> Option<String> {
        self.execute_function("small_image_key").await
    }

    pub async fn small_image_text(&mut self) -> Option<String> {
        self.execute_function("small_image_text").await
    }

    pub async fn start_timestamp(&mut self) -> Option<i64> {
        self.execute_function_and_parse("start_timestamp").await
    }

    pub async fn end_timestamp(&mut self) -> Option<i64> {
        self.execute_function_and_parse("end_timestamp").await
    }

    pub async fn buttons(&mut self) -> Option<Vec<(String, String)>> {
        if let Some(output) = self.execute_function("buttons").await {
            let parts: Vec<&str> = output.split('\u{0091}').collect();
            if parts.len() % 2 == 0 {
                Some(
                    parts
                        .iter()
                        .step_by(2)
                        .zip(parts.iter().skip(1).step_by(2))
                        .map(|(label, url)| (label.to_string(), url.to_string()))
                        .collect(),
                )
            } else {
                None
            }
        } else {
            None
        }
    }
}
