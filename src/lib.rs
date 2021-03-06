// Copyright (c) 2019-2020 Rafael Alcaraz Mercado. All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

pub mod compute;
pub mod computecore;
pub mod computenetwork;
pub mod computestorage;
pub mod hypervdevicevirtualization;

#[cfg(feature = "schema")]
pub mod netschema;

#[cfg(feature = "schema")]
pub mod schema;

pub type HcsResult<T> = Result<T, compute::errorcodes::ResultCode>;
