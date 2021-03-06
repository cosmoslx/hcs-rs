// Copyright  rafawo (rafawo1@hotmail.com). All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

pub mod guest;
pub mod service;
pub mod system;

use serde::{Deserialize, Serialize};

impl std::default::Default for ModifyRequestType {
    fn default() -> Self {
        ModifyRequestType::Add
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ModifyRequestType {
    Add,
    Remove,
    Update,
}
