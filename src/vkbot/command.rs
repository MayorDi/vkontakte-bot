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

use crate::vkbot::context::Context;

/// Abstraction over `Callback`, contains the template for calling the command and contains the executing part.
#[derive(Debug, Clone)]
pub struct Command<'re> {
    pub(crate) regex: &'re str,
    pub(crate) callback: fn(Context),
}

impl<'re> Command<'re> {
    pub fn new(regex: &'re str, callback: fn(Context)) -> Self {
        Self {
            regex,
            callback,
        }
    }
}
