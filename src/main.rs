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

mod config;
mod rich_presence_client;
mod rich_presence_controller;
mod shell;

use std::{
    path::PathBuf,
    sync::{
        mpsc::{channel, Receiver},
        Arc,
    },
    time::Duration,
};

use clap::Clap;
use log::{error, info, warn};
use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use serde_json::from_str;
use simplelog::{ColorChoice, ConfigBuilder, LevelFilter, TermLogger, TerminalMode};
use tokio::{
    task::{self, LocalSet},
    time::sleep,
};

use crate::{config::Config, rich_presence_controller::RichPresenceController, shell::Shell};

const INVALID_UPDATE_RESPONSE_UPDATE_DELAY: u64 = 10;

async fn wait_for_change(rx: Receiver<DebouncedEvent>) -> Receiver<DebouncedEvent> {
    loop {
        if let Ok(DebouncedEvent::Write(_)) = rx.try_recv() {
            break;
        }

        sleep(Duration::from_secs(1)).await;
    }

    rx
}

async fn process_rich_presence(mut config_shell: Shell) {
    let mut controller = RichPresenceController::new();
    let mut is_connected = false;

    loop {
        let sleep_duration;
        let update_response = config_shell.execute("update").await;

        match from_str::<Config>(&update_response) {
            Ok(config) => {
                match controller.update(config.items.into_iter()).await {
                    Ok(()) => {
                        if !is_connected {
                            is_connected = true;

                            info!("Connected to Discord!");
                        }
                    }
                    Err(err) => {
                        is_connected = false;

                        warn!("{} Retrying after {} seconds.", err, config.update_delay);
                    }
                }

                sleep_duration = Duration::from_secs(config.update_delay);
            }
            Err(err) => {
                is_connected = false;

                error!(
                    "Error while parsing config update response: `{}`. Received value: `{}`. Retrying after {} seconds.",
                    err, update_response, INVALID_UPDATE_RESPONSE_UPDATE_DELAY
                );

                sleep_duration = Duration::from_secs(INVALID_UPDATE_RESPONSE_UPDATE_DELAY);
            }
        }

        sleep(sleep_duration).await;
    }
}

#[derive(Clap)]
#[clap(version = "2.0.0", author = "trickybestia <trickybestia@gmail.com>")]
struct Args {
    /// Path to the config file
    #[clap(short, long)]
    config: PathBuf,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    TermLogger::init(
        LevelFilter::Info,
        ConfigBuilder::new().set_time_to_local(true).build(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();

    let local = LocalSet::new();
    let args = Arc::new(Args::parse());
    let (tx, mut rx) = channel();
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(1)).unwrap();

    watcher
        .watch(args.config.as_path(), RecursiveMode::NonRecursive)
        .unwrap();

    local
        .run_until(async {
            loop {
                let shell = Shell::new(args.config.as_path()).await;
                let task = task::spawn_local(async move { process_rich_presence(shell).await });

                rx = wait_for_change(rx).await;

                task.abort();
                info!("Config file changed! Restarting...");
            }
        })
        .await;
}
