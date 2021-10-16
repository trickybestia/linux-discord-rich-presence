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

use std::error::Error;

use discord_rich_presence::{
    activity::{Activity, Assets, Button, Timestamps},
    new_client, DiscordIpc,
};

use crate::config::ConfigItem;

pub struct RichPresenceClient {
    client: Box<dyn DiscordIpc>,
}

impl RichPresenceClient {
    pub fn new(application_id: u64) -> Result<Self, Box<dyn Error>> {
        let mut client =
            Box::new(new_client(&application_id.to_string()).unwrap()) as Box<dyn DiscordIpc>;

        client.connect()?;

        Ok(Self { client })
    }

    pub async fn set_activity(&mut self, config: &ConfigItem) -> Result<(), Box<dyn Error>> {
        let mut timestamps = Timestamps::new();
        let mut assets = Assets::new();
        let mut activity = Activity::new();
        let mut buttons = Vec::new();

        if let Some(state) = &config.state {
            activity = activity.state(state.as_str());
        }
        if let Some(details) = &config.details {
            activity = activity.details(details.as_str());
        }

        if let Some(large_image) = &config.large_image {
            assets = assets.large_image(large_image.key.as_str());

            if let Some(text) = &large_image.text {
                assets = assets.large_text(text.as_str());
            }
        }
        if let Some(small_image) = &config.small_image {
            assets = assets.small_image(small_image.key.as_str());

            if let Some(text) = &small_image.text {
                assets = assets.small_text(text.as_str());
            }
        }

        if let Some(start_timestamp) = config.start_timestamp {
            timestamps = timestamps.start(start_timestamp);
        }
        if let Some(end_timestamp) = config.end_timestamp {
            timestamps = timestamps.start(end_timestamp);
        }

        if config.buttons.len() != 0 {
            for button in &config.buttons {
                buttons.push(Button::new(button.label.as_str(), button.url.as_str()));
            }

            activity = activity.buttons(buttons);
        }

        activity = activity.assets(assets).timestamps(timestamps);

        self.client.set_activity(activity)?;

        Ok(())
    }
}

#[allow(unused_must_use)]
impl Drop for RichPresenceClient {
    fn drop(&mut self) {
        self.client.close();
    }
}
