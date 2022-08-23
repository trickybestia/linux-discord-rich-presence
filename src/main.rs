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

mod process_wrapper;
mod rich_presence_client;
mod rich_presence_config;
mod rich_presence_controller;
mod update_message;

use std::{path::PathBuf, time::Duration};

use clap::Parser;
use lazy_static::lazy_static;
use log::{info, warn};
use rich_presence_config::RichPresenceConfig;
use simplelog::{ColorChoice, ConfigBuilder, LevelFilter, TermLogger, TerminalMode};
use tokio::{
    sync::mpsc::{channel, Receiver},
    time::timeout,
};

use crate::{rich_presence_controller::RichPresenceController, update_message::UpdateMessage};

lazy_static! {
    static ref UPDATE_DELAY: Duration = Duration::from_secs(10);
}

async fn process_rich_presence(mut updates_receiver: Receiver<UpdateMessage>) {
    let mut controller = RichPresenceController::new();
    let mut is_connected = false;
    let mut last_message = UpdateMessage::new();

    loop {
        let message = timeout(*UPDATE_DELAY, updates_receiver.recv()).await;

        match message {
            Ok(Some(message)) => last_message = message,
            _ => (),
        }

        match controller.update(&last_message).await {
            Ok(()) => {
                if !is_connected {
                    is_connected = true;

                    info!("Connected to Discord!");
                }
            }
            Err(err) => {
                is_connected = false;

                warn!("{} Retrying after {} seconds.", err, UPDATE_DELAY.as_secs());
            }
        }
    }
}

#[derive(Parser)]
#[clap(author, version, about)]
struct Args {
    /// Path to the config file
    #[clap(short, long)]
    config: PathBuf,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    TermLogger::init(
        LevelFilter::Info,
        ConfigBuilder::new()
            .set_time_offset_to_local()
            .unwrap()
            .build(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();

    let args = Args::parse();
    let (tx, rx) = channel(10);
    let _config = RichPresenceConfig::new(args.config, tx);

    process_rich_presence(rx).await;
}
