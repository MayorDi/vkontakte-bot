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

use super::VkBot;

/// It is an abstract implementation of the Long Poll server for convenient query generation.
#[derive(Debug, Clone, PartialEq)]
pub struct LongPollServer {
    server: String,
    key: String,
    ts: u64,
}

impl LongPollServer {
    pub fn new(vk_bot: VkBot) -> crate::result::Result<Self> {
        let id = vk_bot.id;
        let token = vk_bot.api_settings.access_token();
        let v = vk_bot.api_settings.version();
        let req_url = format!(
            "https://api.vk.com/method/groups.getLongPollServer?group_id={id}&access_token={token}&v={v}"
        );

        let res = reqwest::blocking::ClientBuilder::new()
            .build()
            .unwrap()
            .get(req_url)
            .send()
            .unwrap()
            .text()
            .unwrap();
        let res: serde_json::Value = serde_json::from_str(res.as_str()).unwrap();

        match res.get("response") {
            Some(res) => {
                let server = res["server"].as_str().unwrap().to_string();
                let key = res["key"].as_str().unwrap().to_string();
                let ts: u64 = res["ts"].as_str().unwrap().to_string().parse().unwrap();

                Ok(Self { server, key, ts })
            }
            None => Err(res),
        }
    }

    /// Update Long Poll structure to detect new events.
    pub fn update(&mut self) -> crate::result::Result<serde_json::Value> {
        let server = &self.server;
        let key = &self.key;
        let ts = &mut self.ts;

        let req_url = format!("{server}?act=a_check&key={key}&ts={ts}&wait=25");
        let res = reqwest::blocking::ClientBuilder::new()
            .build()
            .unwrap()
            .get(req_url.clone())
            .send()
            .unwrap()
            .text()
            .unwrap();
        let res: serde_json::Value = serde_json::from_str(res.as_str()).unwrap();

        match res.get("ts") {
            Some(ts_new) => {
                *ts = ts_new.as_str().unwrap().to_string().parse().unwrap();

                Ok(res.clone())
            }
            None => Err(res),
        }
    }
}
