/*
    Copyright © 2021 trickybestia <trickybestia@gmail.com>

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use chrono::{DateTime, Local, NaiveDateTime, TimeZone, Utc};
use clap::Clap;
use discord_rpc_client::Client;
use serde::Deserialize;
use std::{
    error::Error,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
    process::Command,
    str,
    thread::sleep,
    time::Duration,
};

#[derive(Clap)]
#[clap(version = "0.1.0", author = "trickybestia <trickybestia@gmail.com>")]
struct Args {
    /// Path to the config file
    #[clap(short, long)]
    config: PathBuf,
}

#[derive(Deserialize)]
struct Config {
    rich_presence_application_id: u64,
    update_delay: u64,
    get_state: String,
    get_details: String,
    get_large_image_key: String,
    get_large_image_text: String,
    get_small_image_key: String,
    get_small_image_text: String,
}

fn read_config(path: &Path) -> Result<Config, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(serde_yaml::from_str::<Config>(buffer.as_str())?)
}

fn run_shell(code: &str) -> Option<String> {
    let shell = Command::new("sh").arg("-c").arg(code).output().unwrap();
    let output = str::from_utf8(&shell.stdout[..]).unwrap().to_owned();
    if output.len() == 0 {
        None
    } else {
        Some(output)
    }
}

fn get_startup_time() -> DateTime<Utc> {
    let process_output = Command::new("uptime").arg("-s").output().unwrap();
    let uptime_str = str::from_utf8(&process_output.stdout[..]).unwrap();
    let naive_uptime = NaiveDateTime::parse_from_str(uptime_str, "%Y-%m-%d %H:%M:%S\n").unwrap();
    let uptime = Local
        .from_local_datetime(&naive_uptime)
        .unwrap()
        .with_timezone(&Utc);
    uptime
}

fn main() {
    let args = Args::parse();
    let start_time = get_startup_time().timestamp() as u64;

    match read_config(args.config.as_path()) {
        Ok(config) => {
            let mut client = Client::new(config.rich_presence_application_id);

            client.start();

            loop {
                if let Err(error) = client.set_activity(|mut activity| {
                    if let Some(state) = run_shell(config.get_state.as_str()) {
                        activity = activity.state(state);
                    }
                    if let Some(details) = run_shell(config.get_details.as_str()) {
                        activity = activity.details(details);
                    }
                    activity = activity.assets(|mut assets| {
                        if let Some(large_image_key) =
                            run_shell(config.get_large_image_key.as_str())
                        {
                            assets = assets.large_image(large_image_key);
                        }
                        if let Some(large_image_text) =
                            run_shell(config.get_large_image_text.as_str())
                        {
                            assets = assets.large_text(large_image_text);
                        }
                        if let Some(small_image_key) =
                            run_shell(config.get_small_image_key.as_str())
                        {
                            assets = assets.small_image(small_image_key);
                        }
                        if let Some(small_image_text) =
                            run_shell(config.get_small_image_text.as_str())
                        {
                            assets = assets.small_text(small_image_text);
                        }
                        assets
                    });
                    activity.timestamps(|timestamps| timestamps.start(start_time))
                }) {
                    println!(
                        "Error while setting activity: `{}`. Retrying after {} seconds.",
                        error, config.update_delay
                    );
                }

                sleep(Duration::from_secs(config.update_delay));
            }
        }
        Err(error) => println!("Error while reading config file: `{}`.", error),
    }
}
