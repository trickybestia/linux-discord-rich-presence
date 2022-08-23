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

use std::{path::PathBuf, time::Duration};

use log::{error, info};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use serde_json::from_str;
use tokio::{
    select,
    task::{spawn_blocking, JoinHandle},
};

use crate::{process_wrapper::ProcessWrapper, update_message::UpdateMessage};

pub struct RichPresenceConfig {
    task: JoinHandle<()>,
}

impl RichPresenceConfig {
    async fn run(path: PathBuf, updates_sender: tokio::sync::mpsc::Sender<UpdateMessage>) {
        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(1)).unwrap();

        watcher.watch(&path, RecursiveMode::NonRecursive).unwrap();

        let mut watcher_task;
        let mut process;

        macro_rules! reload_config {
            ($watcher_rx:ident) => {
                watcher_task = spawn_blocking(move || {
                    #[allow(unused_must_use)]
                    {
                        $watcher_rx.recv();
                    }

                    $watcher_rx
                });

                process = ProcessWrapper::new(&path).await;
            };
        }

        reload_config!(rx);

        loop {
            select! {
                returned_rx = &mut watcher_task => {
                    info!("Config file was changed! Restarting...");

                    let returned_rx = returned_rx.unwrap();

                    reload_config!(returned_rx);
                },
                line = process.read_line() => {
                    match line {
                        Ok(Some(line)) => {
                            match from_str::<UpdateMessage>(&line) {
                                Ok(message) => {
                                    if updates_sender.send(message).await.is_err() {
                                        break;
                                    }
                                },
                                Err(err) => {
                                    error!(
                                        "Error while parsing config response: `{}`. Received value: `{}`.",
                                        err, line
                                    );
                                }
                            }
                        }
                        _ => {
                            error!("Config Process' stdout was closed (it died?). Showing last sent activity.");
                        }
                    }
                },
            }
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
