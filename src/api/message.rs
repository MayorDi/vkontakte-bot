// Copyright 2024 Dmitriy Mayorov

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::alias::Id;

use super::ApiSettings;

/// Implements an abstraction for messages that will be sent to a specific user.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub(crate) random_id: Id,
    pub(crate) peer_id: Id,
    pub(crate) message: String,
}

impl Message {
    /// Send a message to the user with certain API settings.
    pub fn send(&self, api_settings: &ApiSettings) -> crate::result::Result<()> {
        let (random_id, peer_id, message, access_token, v) = (
            &self.random_id,
            &self.peer_id,
            &self.message,
            &api_settings.access_token,
            &api_settings.version,
        );

        let req = format!(
            "https://api.vk.com/method/messages.send?random_id={random_id}&peer_id={peer_id}&message={message}&access_token={access_token}&v={v}"
        );

        let res = reqwest::blocking::ClientBuilder::new()
            .build()
            .unwrap()
            .get(req)
            .send()
            .unwrap()
            .text()
            .unwrap();
        let res: serde_json::Value = serde_json::from_str(res.as_str()).unwrap();

        match res.get("response") {
            Some(_) => Ok(()),
            None => Err(res),
        }
    }
}
