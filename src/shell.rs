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

use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    process::{Child, ChildStdin, ChildStdout, Command, Stdio},
};

pub struct Shell {
    process: Child,
    stdin_writer: BufWriter<ChildStdin>,
    stdout_reader: BufReader<ChildStdout>,
}

#[allow(unused_must_use)]
impl Drop for Shell {
    fn drop(&mut self) {
        self.process.kill();
    }
}

impl Shell {
    pub fn new() -> Self {
        let mut process = Command::new("sh")
            .env("PS1", "")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        Self {
            stdin_writer: BufWriter::new(process.stdin.take().unwrap()),
            stdout_reader: BufReader::new(process.stdout.take().unwrap()),
            process,
        }
    }

    pub fn execute(&mut self, code: &str) -> String {
        let mut buf = String::new();

        self.stdin_writer.write_all(code.as_bytes()).unwrap();
        self.stdin_writer.write_all("\n".as_bytes()).unwrap();
        self.stdin_writer.flush().unwrap();

        self.stdout_reader.read_line(&mut buf).unwrap();

        buf.remove(buf.len() - 1);

        buf
    }
}
