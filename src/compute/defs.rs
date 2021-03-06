// Copyright (c) 2019-2020 Rafael Alcaraz Mercado. All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

//! Rust abstractions for the public types and definitions used by the Host Compute APIs.

use winutils_rs::windefs::*;

#[allow(overflowing_literals)]
pub const HCS_E_PROCESS_INFO_NOT_AVAILABLE: HResult = 0x8037011D;

#[allow(overflowing_literals)]
pub const HCS_E_SERVICE_DISCONNECT: HResult = 0x8037011E;

/// Handle to a compute system
pub type HcsSystemHandle = Handle;

/// Handle to a process running in a compute system
pub type HcsProcessHandle = Handle;

/// Handle to an operation on a compute system
pub type HcsOperationHandle = Handle;

/// Handle to a callback registered on a compute system or process handle.
pub type HcsCallbackHandle = Handle;

/// Type of an operation. These correspond to the functions that invoke the operation.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum HcsOperationType {
    None = -1,
    Enumerate = 0,
    Create = 1,
    Start = 2,
    Shutdown = 3,
    Pause = 4,
    Resume = 5,
    Save = 6,
    Terminate = 7,
    Modify = 8,
    GetProperties = 9,
    CreateProcess = 10,
    SignalProcess = 11,
    GetProcessInfo = 12,
    GetProcessProperties = 13,
    ModifyProcess = 14,
}

pub const HCS_INVALID_OPERATION_ID: u64 = std::u64::MAX;

/// Function type for the completion callback of an operation.
pub type HcsOperationCompletion =
    Option<unsafe extern "system" fn(operation: HcsOperationHandle, context: PVoid)>;

/// Events indicated to callbacks registered by HcsRegisterComputeSystemCallback or
/// HcsRegisterProcessCallback (since Windows 1809).
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum HcsEventType {
    Invalid = 0x00000000,

    /// Events for HcsSystemHandle
    SystemExited = 0x00000001,
    SystemCrashInitiated = 0x00000002,
    SystemCrashReport = 0x00000003,
    SystemRdpEnhancedModeStateChanged = 0x00000004,
    SystemSiloJobCreated = 0x00000005,
    SystemGuestConnectionClosed = 0x00000006,

    /// Events for HcsProcessHandle
    ProcessExited = 0x00010000,

    /// Common Events
    OperationCallback = 0x01000000,
    ServiceDisconnect = 0x02000000,
}

/// Provides information about an event that occurred on a compute system or process.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HcsEvent {
    /// Type of Event (see HcsEventType)
    pub event_type: HcsEventType,

    /// Provides additional data for the event.
    pub event_data: PCWStr,

    /// Handle to a completed operation (if Type is HcsEventType::OperationCallback).
    pub operation: HcsOperationHandle,
}

/// Options for an event callback registration
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum HcsEventOptions {
    None = 0x00000000,
    EnableOperationCallbacks = 0x00000001,
}

/// Function type for compute system event callbacks
pub type HcsEventCallback =
    Option<unsafe extern "system" fn(event: *const HcsEvent, context: PVoid)>;

/// Flags applicable to HcsNotifications
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum HcsNotificationFlag {
    Success = 0x00000000,
    Failure = 0x80000000,
}

/// Notifications indicated to callbacks registered by HcsRegisterComputeSystemCallback or
/// HcsRegisterProcessCallback (until Windows 1803).
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum HcsNotifications {
    Invalid = 0x00000000,

    /// Notifications for HcsSystemHandle
    SystemExited = 0x00000001,
    SystemCreateCompleted = 0x00000002,
    SystemStartCompleted = 0x00000003,
    SystemPauseCompleted = 0x00000004,
    SystemResumeCompleted = 0x00000005,
    SystemCrashReport = 0x00000006,
    SystemSiloJobCreated = 0x00000007,
    SystemSaveCompleted = 0x00000008,
    SystemRdpEnhancedModeStateChanged = 0x00000009,
    SystemShutdownFailed = 0x0000000A,
    SystemGetPropertiesCompleted = 0x0000000B,
    SystemModifyCompleted = 0x0000000C,
    SystemCrashInitiated = 0x0000000D,
    SystemGuestConnectionClosed = 0x0000000E,

    /// Notifications for HcsProcessHandle
    ProcessExited = 0x00010000,

    /// Common notifications
    ServiceDisconnect = 0x01000000,

    /// The upper 4 bits are reserved.
    FlagsReserved = 0xF0000000,
}

/// Function type for compute system notification callbacks
#[allow(non_camel_case_types)]
pub type HcsNotificationCallback = Option<
    unsafe extern "system" fn(
        notification_type: DWord,
        context: PVoid,
        notifications_status: HResult,
        notification_data: PCWStr,
    ),
>;

/// Struct containing information about a process created by HcsStartProcessInComputeSystem
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HcsProcessInformation {
    /// Identifier of the created process
    pub process_id: DWord,
    reserved: DWord,

    /// If created, standard input handle of the process
    pub std_input: Handle,

    /// If created, standard output handle of the process
    pub std_output: Handle,

    /// If created, standard error handle of the process
    pub std_error: Handle,
}
