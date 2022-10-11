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

use std::path::{Path, PathBuf};

use is_executable::is_executable;
use log::{error, info};
use notify::{event::AccessKind, Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use serde_json::from_str;
use tokio::{fs::read_to_string, spawn, sync::mpsc, task::JoinHandle};

use crate::{process_wrapper::ProcessWrapper, update_message::UpdateMessage};

async fn load_config<S>(path: S) -> Option<UpdateMessage>
where
    S: AsRef<Path>,
{
    match read_to_string(path).await {
        Ok(config) => match from_str::<UpdateMessage>(&config) {
            Ok(message) => return Some(message),
            Err(err) => error!(
                "Error while parsing config file: `{}`. Config: `{}`.",
                err, config
            ),
        },
        Err(err) => error!("Error while reading config file: `{}`.", err),
    }

    None
}

pub struct RichPresenceConfig {
    task: JoinHandle<()>,
}

impl RichPresenceConfig {
    async fn read(path: PathBuf, updates_sender: tokio::sync::mpsc::Sender<UpdateMessage>) {
        let mut process = ProcessWrapper::new(path).await;

        while let Ok(Some(line)) = process.read_line().await {
            match from_str::<UpdateMessage>(&line) {
                Ok(message) => {
                    if updates_sender.send(message).await.is_err() {
                        break;
                    }
                }
                Err(err) => {
                    error!(
                        "Error while parsing config response: `{}`. Received value: `{}`.",
                        err, line
                    );
                }
            }
        }

        info!("Config Process' stdout was closed (it died?). Showing last sent activity.");
    }

    async fn run(path: PathBuf, updates_sender: tokio::sync::mpsc::Sender<UpdateMessage>) {
        let (tx, mut rx) = mpsc::channel(10);
        let mut watcher = RecommendedWatcher::new(
            move |event: notify::Result<notify::Event>| {
                let event = event.unwrap();

                if let EventKind::Access(AccessKind::Close(_)) = event.kind {
                    tx.blocking_send(()).unwrap();
                }
            },
            Config::default(),
        )
        .unwrap();

        watcher.watch(&path, RecursiveMode::NonRecursive).unwrap();

        let mut _reader_task: Option<JoinHandle<()>> = None;

        macro_rules! reload_config {
            () => {
                if let Some(_reader_task) = _reader_task.take() {
                    _reader_task.abort();
                }

                if is_executable(&path) {
                    _reader_task = Some(spawn(Self::read(path.clone(), updates_sender.clone())));
                } else if let Some(message) = load_config(&path).await {
                    if updates_sender.send(message).await.is_err() {
                        return;
                    }
                }
            };
        }

        reload_config!();

        loop {
            rx.recv().await;

            info!("Config file was changed! Restarting...");

            reload_config!();
        }
    }

    pub fn new(path: PathBuf, updates_sender: tokio::sync::mpsc::Sender<UpdateMessage>) -> Self {
        Self {
            task: tokio::spawn(RichPresenceConfig::run(path, updates_sender)),
        }
    }
}

impl Drop for RichPresenceConfig {
    fn drop(&mut self) {
        self.task.abort()
    }
}
