/*
    Copyright © 2021-2022 trickybestia <trickybestia@gmail.com>

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

use std::{ffi::OsStr, process::Stdio};

use tokio::{
    io::{self, AsyncBufReadExt, BufReader, Lines},
    process::{Child, ChildStdout, Command},
};

pub struct ProcessWrapper {
    process: Child,
    stdout_lines: Lines<BufReader<ChildStdout>>,
}

#[allow(unused_must_use)]
impl Drop for ProcessWrapper {
    fn drop(&mut self) {
        self.process.kill();
    }
}

impl ProcessWrapper {
    pub async fn new<S>(program: S) -> Self
    where
        S: AsRef<OsStr>,
    {
        let mut process = Command::new(program)
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        Self {
            stdout_lines: BufReader::new(process.stdout.take().unwrap()).lines(),
            process,
        }
    }

    pub async fn read_line(&mut self) -> io::Result<Option<String>> {
        self.stdout_lines.next_line().await
    }
}
