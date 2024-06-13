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

use crate::alias::{Id, VersionApi};

pub mod message;
mod type_event;

pub use type_event::TypeEvent;

#[derive(Debug, Clone, PartialEq)]
pub struct Event {
    pub(crate) type_event: TypeEvent,
    pub(crate) event_id: String,
    pub(crate) version_api: VersionApi,
    pub(crate) group_id: Id,
    pub(crate) object: serde_json::Value,
}

impl From<&serde_json::Value> for Event {
    fn from(value: &serde_json::Value) -> Self {
        Self {
            type_event: TypeEvent::from(value["type"].as_str().unwrap().to_string()),
            event_id: value["event_id"].as_str().unwrap().to_string(),
            version_api: value["v"].as_str().unwrap().to_string(),
            group_id: value["group_id"].as_u64().unwrap(),
            object: value["object"].clone(),
        }
    }
}
