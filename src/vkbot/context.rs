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

use crate::{alias::Id, api::ApiSettings};

#[derive(Debug, Clone)]
pub struct Context {
    ctx_id: Id,
    api_settings: ApiSettings,
}

impl Context {
    pub(crate) fn new(ctx_id: Id, api_settings: ApiSettings) -> Self {
        Self {
            ctx_id,
            api_settings,
        }
    }

    pub fn reply(&self, message: &str) -> crate::result::Result<()> {
        let message = crate::api::message::Message {
            random_id: 0,
            peer_id: self.ctx_id,
            message: message.to_string(),
        };

        message.send(&self.api_settings)
    }
}
