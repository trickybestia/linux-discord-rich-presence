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

use serde::Deserialize;

pub type UpdateMessage = Vec<UpdateMessageItem>;

#[derive(Deserialize)]
pub struct UpdateMessageItem {
    pub application_id: u64,
    #[serde(default)]
    pub state: Option<String>,
    #[serde(default)]
    pub details: Option<String>,
    #[serde(default)]
    pub large_image: Option<Image>,
    #[serde(default)]
    pub small_image: Option<Image>,
    #[serde(default)]
    pub start_timestamp: Option<i64>,
    #[serde(default)]
    pub end_timestamp: Option<i64>,
    #[serde(default)]
    pub buttons: Vec<Button>,
}

#[derive(Deserialize)]
pub struct Button {
    pub label: String,
    pub url: String,
}

#[derive(Deserialize)]
pub struct Image {
    pub key: String,
    #[serde(default)]
    pub text: Option<String>,
}
