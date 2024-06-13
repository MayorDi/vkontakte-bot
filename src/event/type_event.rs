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

use crate::impl_from_string;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeEvent {
    MessageTypingState,
    WallPostNew,
    MessageNew,
    None,
}

impl ToString for TypeEvent {
    fn to_string(&self) -> String {
        match self {
            Self::WallPostNew => String::from("wall_post_new"),
            Self::MessageNew => String::from("message_new"),
            Self::MessageTypingState => String::from("message_typing_state"),
            Self::None => String::from("_"),
        }
    }
}

impl_from_string!(
    "wall_post_new" -> Self::WallPostNew,
    "message_new" -> Self::MessageNew,
    "message_typing_state" -> Self::MessageTypingState
);

#[macro_export]
macro_rules! impl_from_string {
    ($($from:literal -> $to:path),+) => {
        impl From<String> for TypeEvent {
            fn from(value: String) -> Self {
                $(
                    if value == $from.to_owned() {
                        return $to;
                    }
                )+

                TypeEvent::None
            }
        }
    };
}
