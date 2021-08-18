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
use discord_rpc_client::Client;
use std::{path::PathBuf, thread::sleep, time::Duration};

#[derive(Clap)]
#[clap(version = "0.1.0", author = "trickybestia <trickybestia@gmail.com>")]
struct Args {
    /// Path to the config file
    #[clap(short, long)]
    config: PathBuf,
}

fn main() {
    let args = Args::parse();

    let mut config_shell = ConfigShell::new(args.config.as_path());

    let mut client = Client::new(config_shell.application_id().unwrap());

    client.start();

    loop {
        if let Err(error) = client.set_activity(|mut activity| {
            if let Some(state) = config_shell.state() {
                activity = activity.state(state);
            }
            if let Some(details) = config_shell.details() {
                activity = activity.details(details);
            }
            activity = activity.assets(|mut assets| {
                if let Some(large_image_key) = config_shell.large_image_key() {
                    assets = assets.large_image(large_image_key);
                }
                if let Some(large_image_text) = config_shell.large_image_text() {
                    assets = assets.large_text(large_image_text);
                }
                if let Some(small_image_key) = config_shell.small_image_key() {
                    assets = assets.small_image(small_image_key);
                }
                if let Some(small_image_text) = config_shell.small_image_text() {
                    assets = assets.small_text(small_image_text);
                }
                assets
            });
            activity = activity.timestamps(|mut timestamps| {
                if let Some(start_timestamp) = config_shell.start_timestamp() {
                    timestamps = timestamps.start(start_timestamp);
                }
                if let Some(end_timestamp) = config_shell.end_timestamp() {
                    timestamps = timestamps.start(end_timestamp);
                }
                timestamps
            });
            activity
        }) {
            println!(
                "Error while setting activity: `{}`. Retrying after {} seconds.",
                error,
                config_shell.update_delay().unwrap()
            );
        }

        sleep(Duration::from_secs(config_shell.update_delay().unwrap()));
    }
}
