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

use serde::Deserialize;

pub type UpdateMessage = Vec<UpdateMessageItem>;

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct UpdateMessageItem {
    pub application_id: u64,
    pub state: Option<String>,
    pub details: Option<String>,
    pub large_image: Option<Image>,
    pub small_image: Option<Image>,
    pub start_timestamp: Option<i64>,
    pub end_timestamp: Option<i64>,
    pub buttons: Vec<Button>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct Button {
    pub label: String,
    pub url: String,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct Image {
    pub key: String,
    pub text: Option<String>,
}
