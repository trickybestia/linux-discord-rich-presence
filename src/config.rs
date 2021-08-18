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
use std::{fs::File, io::Read, path::Path, str::FromStr};

pub struct ConfigShell {
    shell: Shell,
}

impl ConfigShell {
    pub fn new(config_path: &Path) -> Self {
        let mut shell = Shell::new();
        let mut buf = String::new();

        File::open(config_path)
            .unwrap()
            .read_to_string(&mut buf)
            .unwrap();

        shell.execute(buf.as_str());

        Self { shell }
    }

    fn execute_function(&mut self, function_name: &str) -> Option<String> {
        let output = self.shell.execute(function_name);

        if output.is_empty() {
            None
        } else {
            Some(output)
        }
    }

    fn execute_function_and_parse<T: FromStr>(&mut self, function_name: &str) -> Option<T> {
        let output = self.shell.execute(function_name);

        if let Ok(value) = output.parse() {
            Some(value)
        } else {
            None
        }
    }

    pub fn application_id(&mut self) -> Option<u64> {
        self.execute_function_and_parse("application_id")
    }

    pub fn update_delay(&mut self) -> Option<u64> {
        self.execute_function_and_parse("update_delay")
    }

    pub fn state(&mut self) -> Option<String> {
        self.execute_function("state")
    }

    pub fn details(&mut self) -> Option<String> {
        self.execute_function("details")
    }

    pub fn large_image_key(&mut self) -> Option<String> {
        self.execute_function("large_image_key")
    }

    pub fn large_image_text(&mut self) -> Option<String> {
        self.execute_function("large_image_text")
    }

    pub fn small_image_key(&mut self) -> Option<String> {
        self.execute_function("small_image_key")
    }

    pub fn small_image_text(&mut self) -> Option<String> {
        self.execute_function("small_image_text")
    }

    pub fn start_timestamp(&mut self) -> Option<u64> {
        self.execute_function_and_parse("start_timestamp")
    }

    pub fn end_timestamp(&mut self) -> Option<u64> {
        self.execute_function_and_parse("end_timestamp")
    }
}
