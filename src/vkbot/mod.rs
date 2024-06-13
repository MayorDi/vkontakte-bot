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

use crate::{
    api::ApiSettings,
    error::Error,
    event::{Event, TypeEvent},
};

mod command;
mod context;
mod long_poll_server;

#[derive(Debug, Clone)]
pub struct VkBot {
    pub id: u32,
    pub api_settings: ApiSettings,
    long_poll_server: Option<LongPollServer>,
    commands: Vec<Command>,
}

impl VkBot {
    pub fn new(id: u32, api_settings: ApiSettings) -> Self {
        Self {
            id,
            api_settings,
            long_poll_server: None,
            commands: vec![],
        }
    }

    pub fn init(mut self) -> crate::result::Result<Self> {
        self.long_poll_server = Some(LongPollServer::new(self.clone())?);

        Ok(self)
    }

    pub fn command(&mut self, pattern: &str, callback: fn(Context)) {
        self.commands.push(Command::new(pattern, callback));
    }

    pub fn run(self) -> std::result::Result<(), Error> {
        match self.long_poll_server {
            Some(mut long_poll_server) => loop {
                let res = long_poll_server.update();

                match res {
                    Ok(res) => {
                        let events = Self::generate_events(res);

                        for event in events.iter() {
                            match event.type_event {
                                TypeEvent::MessageNew => {
                                    let message = crate::event::message::Message::from(
                                        event.object["message"].clone(),
                                    );

                                    for cmd in self.commands.iter() {
                                        if cmd.pattern != message.text {
                                            continue;
                                        }

                                        let ctx = Context::new(
                                            message.from_id,
                                            self.api_settings.clone(),
                                        );

                                        (cmd.callback)(ctx);
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    Err(e) => return Err(Error::JsonError(e)),
                }
            },

            None => return Err(Error::VkBotIsNotInit),
        }
    }

    fn generate_events(json: serde_json::Value) -> Vec<Event> {
        let events = json["updates"].as_array().unwrap().clone();
        let mut res = vec![];

        for event in events.iter() {
            res.push(Event::from(event));
        }

        res
    }
}
