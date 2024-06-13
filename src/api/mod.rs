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

use crate::{alias::VersionApi, consts::DEFAULT_VERSION};

pub mod message;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ApiSettings {
    version: VersionApi,
    access_token: String,
}

impl ApiSettings {
    pub fn new() -> Self {
        Self {
            version: DEFAULT_VERSION.to_string(),
            ..Default::default()
        }
    }

    pub fn set_version(mut self, value: impl ToString) -> Self {
        self.version = value.to_string();

        self
    }

    pub fn set_access_token(mut self, value: impl ToString) -> Self {
        self.access_token = value.to_string();

        self
    }

    pub fn access_token(&self) -> &String {
        &self.access_token
    }

    pub fn version(&self) -> &String {
        &self.version
    }
}
