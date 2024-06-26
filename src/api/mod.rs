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

//! The module describes API commands for working with the VKontakte social network.
#![macro_use]
use crate::alias::VersionApi;

pub mod message;

/// Describes the basic settings of the VK API.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ApiSettings {
    version: VersionApi,
    access_token: String,
}

impl ApiSettings {
    pub fn new<T: ToString>(access_token: T, version: T) -> Self {
        Self {
            version: version.to_string(),
            access_token: access_token.to_string()
        }
    }

    /// Get the installed VK API access key.
    pub fn access_token(&self) -> &String {
        &self.access_token
    }

    /// Get the installed version of the VK API.
    pub fn version(&self) -> &String {
        &self.version
    }
}
