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

use std::error::Error;

use discord_rich_presence::{
    activity::{Activity, Assets, Button, Party, Timestamps},
    DiscordIpc, DiscordIpcClient,
};

use crate::update_message::UpdateMessageItem;

pub struct RichPresenceClient {
    client: DiscordIpcClient,
}

impl RichPresenceClient {
    pub fn new(application_id: u64) -> Result<Self, Box<dyn Error>> {
        let mut client = DiscordIpcClient::new(&application_id.to_string())?;

        client.connect()?;

        Ok(Self { client })
    }

    pub async fn set_activity(
        &mut self,
        message: &UpdateMessageItem,
    ) -> Result<(), Box<dyn Error>> {
        let mut timestamps = Timestamps::new();
        let mut assets = Assets::new();
        let mut activity = Activity::new();
        let mut buttons = Vec::new();
        let mut party = Party::new();

        if let Some(state) = &message.state {
            activity = activity.state(state.as_str());
        }
        if let Some(details) = &message.details {
            activity = activity.details(details.as_str());
        }

        if let Some(large_image) = &message.large_image {
            assets = assets.large_image(large_image.key.as_str());

            if let Some(text) = &large_image.text {
                assets = assets.large_text(text.as_str());
            }
        }
        if let Some(small_image) = &message.small_image {
            assets = assets.small_image(small_image.key.as_str());

            if let Some(text) = &small_image.text {
                assets = assets.small_text(text.as_str());
            }
        }

        if let Some(start_timestamp) = message.start_timestamp {
            timestamps = timestamps.start(start_timestamp);
        }
        if let Some(end_timestamp) = message.end_timestamp {
            timestamps = timestamps.start(end_timestamp);
        }

        if let Some(party_size) = message.party {
            party = party.size(party_size);
        }

        if !message.buttons.is_empty() {
            for button in &message.buttons {
                buttons.push(Button::new(button.label.as_str(), button.url.as_str()));
            }

            activity = activity.buttons(buttons);
        }

        activity = activity.assets(assets).timestamps(timestamps).party(party);

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
