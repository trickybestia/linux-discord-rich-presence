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

use std::{collections::HashMap, error::Error};

use crate::{rich_presence_client::RichPresenceClient, update_message::UpdateMessage};

#[derive(thiserror::Error, Debug)]
pub enum UpdateError {
    #[error("Error while connecting to Discord: `{0}`.")]
    Connecting(Box<dyn Error>),
    #[error("Error while setting activity: `{0}`.")]
    ActivitySetting(Box<dyn Error>),
}

pub struct RichPresenceController {
    clients: HashMap<u64, RichPresenceClient>,
}

impl RichPresenceController {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
        }
    }

    pub async fn update(&mut self, message: &UpdateMessage) -> Result<(), UpdateError> {
        let mut new_clients = HashMap::new();

        for item in message {
            let mut client = if let Some(client) = self.clients.remove(&item.application_id) {
                client
            } else {
                RichPresenceClient::new(item.application_id)
                    .map_err(|err| UpdateError::Connecting(err))?
            };

            client
                .set_activity(item)
                .await
                .map_err(|err| UpdateError::ActivitySetting(err))?;

            new_clients.insert(item.application_id, client);
        }

        self.clients = new_clients;

        Ok(())
    }
}
