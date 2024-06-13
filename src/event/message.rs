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

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub struct Message {
    pub(crate) date: u64,
    pub(crate) from_id: Id,
    pub(crate) id: Id,
    pub(crate) out: i64,
    pub(crate) version: i64,
    pub(crate) conversation_message_id: i64,
    pub(crate) important: bool,
    pub(crate) is_hidden: bool,
    pub(crate) peer_id: Id,
    pub(crate) random_id: i64,
    pub(crate) text: String,
}

impl From<serde_json::Value> for Message {
    fn from(value: serde_json::Value) -> Self {
        serde_json::from_value(value).unwrap()
    }
}
