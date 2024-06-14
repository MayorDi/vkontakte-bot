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

use command::Command;
use context::Context;
use long_poll_server::LongPollServer;
use regex::Regex;

use crate::{
    api::ApiSettings,
    error::Error,
    event::{Event, TypeEvent},
};

mod command;
mod context;
mod long_poll_server;

/// It is the main structure in which all action takes place.
/// Connects to the VK server, contains the basic settings of the used `API`, as well as a list of bot commands.
/// ## Example
/// ``` rust
/// use vkontakte_bot::{api::ApiSettings, vkbot::VkBot};
///
/// let access_token = "your token";
/// let group_id = 0; // id of your group
///
/// let api_settings = ApiSettings::new()
///     .set_access_token(access_token)
///     .set_version("5.99");
///
/// let mut vk_bot = VkBot::new(group_id, api_settings);
/// ```
#[derive(Debug, Clone)]
pub struct VkBot<'re> {
    pub id: u32,
    pub api_settings: ApiSettings,
    long_poll_server: Option<LongPollServer>,
    commands: Vec<Command<'re>>,
}

impl<'re> VkBot<'re> {
    pub fn new(id: u32, api_settings: ApiSettings) -> Self {
        Self {
            id,
            api_settings,
            long_poll_server: None,
            commands: vec![],
        }
    }

    /// Initializing the Long Poll server.
    pub fn init(mut self) -> crate::result::Result<Self> {
        self.long_poll_server = Some(LongPollServer::new(self.clone())?);

        Ok(self)
    }

    /// Adding new commands using `Regex`.
    pub fn command(&mut self, regex: &'re str, callback: fn(Context)) {
        self.commands.push(Command::new(regex, callback));
    }

    /// Run the bot.
    pub fn run(self) -> std::result::Result<(), Error> {
        if let None = self.long_poll_server {
            return Err(Error::VkBotIsNotInit);
        }

        let mut long_poll_server = self.long_poll_server.unwrap();
        loop {
            let res = long_poll_server.update();

            if let Err(e) = res {
                return Err(Error::JsonError(e));
            }

            let events = Self::generate_events(res.unwrap());

            for event in events.iter() {
                if let TypeEvent::MessageNew = event.type_event {
                    let message =
                        crate::event::message::Message::from(event.object["message"].clone());

                    for cmd in self.commands.iter() {
                        let re = Regex::new(cmd.regex).unwrap();
                        let captures = re.captures(&message.text);

                        if let None = captures {
                            continue;
                        }

                        let ctx = Context {
                            api_settings: self.api_settings.clone(),
                            captures: captures.unwrap(),
                            message: message.clone(),
                        };

                        (cmd.callback)(ctx);
                        break;
                    }
                }
            }
        }
    }

    /// Creates a list of events as an abstraction over the `JSON` format.
    fn generate_events(json: serde_json::Value) -> Vec<Event> {
        let events = json["updates"].as_array().unwrap().clone();
        let mut res = vec![];

        for event in events.iter() {
            res.push(Event::from(event));
        }

        res
    }
}
