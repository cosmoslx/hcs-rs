// Copyright  rafawo (rafawo1@hotmail.com). All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

use crate::schema::utils::is_default;
use crate::schema::utils::*;
use serde::{Deserialize, Serialize};

impl std::default::Default for FlexibleIoDeviceHostingModel {
    fn default() -> Self {
        FlexibleIoDeviceHostingModel::Internal
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum FlexibleIoDeviceHostingModel {
    Internal,
    External,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct FlexibleIoDevice {
    #[serde(default, rename = "EmulatorId", skip_serializing_if = "is_default")]
    pub emulator_id: GuidSerde,

    #[serde(default, rename = "HostingModel", skip_serializing_if = "is_default")]
    pub hosting_model: FlexibleIoDeviceHostingModel,

    #[serde(default, rename = "Configuration", skip_serializing_if = "is_default")]
    pub configuration: Vec<String>,
}
