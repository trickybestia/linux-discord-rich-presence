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
mod shell;

use clap::Clap;
use config::ConfigShell;
use discord_rich_presence::{
    activity::{Activity, Assets, Button, Timestamps},
    new_client, DiscordIpc,
};
use log::{info, warn};
use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use simplelog::{ColorChoice, ConfigBuilder, LevelFilter, TermLogger, TerminalMode};
use std::{
    error::Error,
    path::PathBuf,
    sync::{
        mpsc::{channel, Receiver},
        Arc,
    },
    time::Duration,
};
use tokio::{
    task::{self, LocalSet},
    time::sleep,
};

async fn wait_for_change(rx: Receiver<DebouncedEvent>) -> Receiver<DebouncedEvent> {
    loop {
        if let Ok(DebouncedEvent::Write(_)) = rx.try_recv() {
            break;
        }

        sleep(Duration::from_secs(1)).await;
    }

    rx
}

async fn process_rich_presence(mut config_shell: ConfigShell) {
    let mut client = new_client(
        config_shell
            .application_id()
            .await
            .unwrap()
            .to_string()
            .as_str(),
    )
    .unwrap();
    let mut is_connected = false;

    loop {
        if !is_connected {
            if let Err(err) = client.connect() {
                warn!(
                    "Error while connecting to Discord: `{:?}`. Retrying after {:?} seconds.",
                    err,
                    config_shell.update_delay().await.unwrap()
                );
            } else {
                is_connected = true;
                info!("Connected to Discord!");
            }
        }

        if is_connected {
            if let Err(err) = set_activity(&mut client, &mut config_shell).await {
                warn!(
                    "Error while setting activity: `{:?}`. Retrying after {:?} seconds.",
                    err,
                    config_shell.update_delay().await.unwrap()
                );
                client.close().unwrap();
                is_connected = false;
            }
        }

        sleep(Duration::from_secs(
            config_shell.update_delay().await.unwrap(),
        ))
        .await;
    }
}

async fn set_activity(
    client: &mut impl DiscordIpc,
    config_shell: &mut ConfigShell,
) -> Result<(), Box<dyn Error>> {
    let mut timestamps = Timestamps::new();
    let mut assets = Assets::new();
    let mut activity = Activity::new();
    let mut buttons = Vec::new();

    let state = config_shell.state().await;
    let details = config_shell.details().await;
    let large_image_key = config_shell.large_image_key().await;
    let large_image_text = config_shell.large_image_text().await;
    let small_image_key = config_shell.small_image_key().await;
    let small_image_text = config_shell.small_image_text().await;
    let start_timestamp = config_shell.start_timestamp().await;
    let end_timestamp = config_shell.end_timestamp().await;
    let raw_buttons = config_shell.buttons().await;

    if let Some(state) = &state {
        activity = activity.state(state.as_str());
    }
    if let Some(details) = &details {
        activity = activity.details(details.as_str());
    }
    if let Some(large_image_key) = &large_image_key {
        assets = assets.large_image(large_image_key.as_str());
    }
    if let Some(large_image_text) = &large_image_text {
        assets = assets.large_text(large_image_text.as_str());
    }
    if let Some(small_image_key) = &small_image_key {
        assets = assets.small_image(small_image_key.as_str());
    }
    if let Some(small_image_text) = &small_image_text {
        assets = assets.small_text(small_image_text.as_str());
    }

    if let Some(start_timestamp) = start_timestamp {
        timestamps = timestamps.start(start_timestamp);
    }
    if let Some(end_timestamp) = end_timestamp {
        timestamps = timestamps.start(end_timestamp);
    }

    if let Some(raw_buttons) = &raw_buttons {
        for raw_button in raw_buttons {
            buttons.push(Button::new(raw_button.0.as_str(), raw_button.1.as_str()));
        }

        activity = activity.buttons(buttons);
    }

    activity = activity.assets(assets).timestamps(timestamps);

    client.set_activity(activity)?;

    Ok(())
}

#[derive(Clap)]
#[clap(version = "0.1.0", author = "trickybestia <trickybestia@gmail.com>")]
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
                let args = args.clone();
                let _task = task::spawn_local(async move {
                    process_rich_presence(ConfigShell::new(args.config.as_path()).await).await
                });

                rx = wait_for_change(rx).await;

                _task.abort();
                info!("Config file changed! Restarting...");
            }
        })
        .await;
}
