// Copyright (c) 2019-2020 Rafael Alcaraz Mercado. All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

//! This module contains types definitions and Rust FFI bindings APIs to interact with the Hyper-V device virtualization support.

use super::defs::*;
use crate::compute::defs::*;
use winutils_rs::windefs::*;

#[link(name = "vmdevicehost")]
extern "system" {
    pub fn HdvInitializeDeviceHost(
        computeSystem: HcsSystemHandle,
        deviceHostHandle: *mut HdvHostHandle,
    ) -> HResult;

    pub fn HdvTeardownDeviceHost(deviceHostHandle: HdvHostHandle) -> HResult;

    pub fn HdvCreateDeviceInstance(
        deviceHostHandle: HdvHostHandle,
        deviceType: HdvDeviceType,
        deviceClassId: *const Guid,
        deviceInstanceId: *const Guid,
        deviceInterface: *const Void,
        deviceContext: *mut Void,
        deviceHandle: *mut HdvDeviceHandle,
    ) -> HResult;

    pub fn HdvReadGuestMemory(
        requestor: HdvDeviceHandle,
        guestPhysicalAddress: u64,
        ByteCount: u32,
        buffer: *mut Byte,
    ) -> HResult;

    pub fn HdvWriteGuestMemory(
        requestor: HdvDeviceHandle,
        guestPhysicalAddress: u64,
        ByteCount: u32,
        buffer: *const Byte,
    ) -> HResult;

    pub fn HdvCreateGuestMemoryAperture(
        requestor: HdvDeviceHandle,
        guestPhysicalAddress: u64,
        ByteCount: u32,
        writeProtected: Bool,
        mappedAddress: *mut PVoid,
    ) -> HResult;

    pub fn HdvDestroyGuestMemoryAperture(
        requestor: HdvDeviceHandle,
        mappedAddress: PVoid,
    ) -> HResult;

    pub fn HdvDeliverGuestInterrupt(
        requestor: HdvDeviceHandle,
        msiAddress: u64,
        msiData: u32,
    ) -> HResult;

    #[cfg(any(feature = "19h1"))]
    pub fn HdvRegisterDoorbellPage(
        requestor: HdvDeviceHandle,
        barIndex: HdvPciBarSelector,
        pageIndex: u64,
        doorbellEvent: Handle,
    ) -> HResult;

    #[cfg(any(feature = "19h1"))]
    pub fn HdvUnregisterDoorbellPage(
        requestor: HdvDeviceHandle,
        barIndex: HdvPciBarSelector,
        pageIndex: u64,
    ) -> HResult;
}
