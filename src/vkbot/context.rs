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

use regex::Captures;

use crate::{api::ApiSettings, event::message::Message};

/// The context in which the event occurred.
#[derive(Debug)]
pub struct Context<'c> {
    pub(crate) api_settings: ApiSettings,
    pub captures: Captures<'c>,
    pub message: Message,
}

impl<'c> Context<'c> {
    /// Send a message in response.
    pub fn reply(&self, message: &str) -> crate::result::Result<()> {
        let message = crate::api::message::Message {
            random_id: 0,
            peer_id: self.message.from_id,
            message: message.to_string(),
        };

        message.send(&self.api_settings)
    }

    /// Get the result of a regular expression.
    pub fn get_captures(&'c self) -> &'c Captures<'c> {
        &self.captures
    }
}
