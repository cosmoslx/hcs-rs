// Copyright  rafawo (rafawo1@hotmail.com). All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

use crate::schema;
use crate::schema::utils::is_default;
use serde::{Deserialize, Serialize};

impl std::default::Default for State {
    fn default() -> Self {
        State::Created
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum State {
    Created,
    Running,
    Paused,
    Stopped,
    SavedAsTemplate,
}

impl std::default::Default for OsType {
    fn default() -> Self {
        OsType::Windows
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum OsType {
    Windows,
    Linux,
}

impl std::default::Default for SystemType {
    fn default() -> Self {
        SystemType::Container
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum SystemType {
    Container,
    VirtualMachine,
    Host,
}

impl std::default::Default for NotificationType {
    fn default() -> Self {
        NotificationType::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum NotificationType {
    None,
    GracefulExit,
    ForcedExit,
    UnexpectedExit,
    Unknown,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct VirtualNodeInfo {
    #[serde(rename = "VirtualNodeIndex")]
    pub virtual_node_index: u8,

    #[serde(rename = "PhysicalNodeNumber")]
    pub physical_node_number: u8,

    #[serde(rename = "VirtualProcessorCount")]
    pub virtual_processor_count: u32,

    #[serde(rename = "MemoryUsageInPages")]
    pub memory_usage_in_pages: u64,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct VmMemory {
    #[serde(rename = "AvailableMemory")]
    pub available_memory: i32,

    #[serde(rename = "AvailableMemoryBuffer")]
    pub available_memory_buffer: i32,

    #[serde(rename = "ReservedMemory")]
    pub reserved_memory: u64,

    #[serde(rename = "AssignedMemory")]
    pub assigned_memory: u64,

    #[serde(rename = "SlpActive")]
    pub slp_active: bool,

    #[serde(rename = "BalancingEnabled")]
    pub balancing_enabled: bool,

    #[serde(rename = "DmOperationInProgress")]
    pub dm_operation_in_progress: bool,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct MemoryInformationForVm {
    #[serde(rename = "VirtualNodeCount")]
    pub virtual_node_count: u8,

    #[serde(rename = "VirtualMachineMemory")]
    pub virtual_machine_memory: VmMemory,

    #[serde(default, rename = "VirtualNodes", skip_serializing_if = "is_default")]
    pub virtual_nodes: Vec<VirtualNodeInfo>,
}

/// CPU runtime statistics
#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ProcessorStats {
    #[serde(rename = "TotalRuntime100ns")]
    pub total_runtime100ns: u64,

    #[serde(
        default,
        rename = "RuntimeUser100ns",
        skip_serializing_if = "is_default"
    )]
    pub runtime_user100ns: u64,

    #[serde(
        default,
        rename = "RuntimeKernel100ns",
        skip_serializing_if = "is_default"
    )]
    pub runtime_kernel100ns: u64,
}

/// Memory runtime statistics
#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct MemoryStats {
    #[serde(rename = "MemoryUsageCommitBytes")]
    pub memory_usage_commit_bytes: u64,

    #[serde(rename = "MemoryUsageCommitPeakBytes")]
    pub memory_usage_commit_peak_bytes: u64,

    #[serde(rename = "MemoryUsagePrivateWorkingSetBytes")]
    pub memory_usage_private_working_set_bytes: u64,
}

/// Storage runtime statistics
#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StorageStats {
    #[serde(rename = "ReadCountNormalized")]
    pub read_count_normalized: u64,

    #[serde(rename = "ReadSizeBytes")]
    pub read_size_bytes: u64,

    #[serde(rename = "WriteCountNormalized")]
    pub write_count_normalized: u64,

    #[serde(rename = "WriteSizeBytes")]
    pub write_size_bytes: u64,
}

/// Runtime statistics for a container
#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Statistics {
    #[serde(rename = "Timestamp")]
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "ContainerStartTime")]
    pub container_start_time: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "Uptime100ns")]
    pub uptime100ns: u64,

    #[serde(rename = "Processor")]
    pub processor: ProcessorStats,

    #[serde(rename = "Memory")]
    pub memory: MemoryStats,

    #[serde(rename = "Storage")]
    pub storage: StorageStats,
}

/// Information about a process running in a container
#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ProcessDetails {
    #[serde(rename = "ProcessId")]
    pub process_id: u32,

    #[serde(rename = "ImageName")]
    pub image_name: String,

    #[serde(rename = "CreateTimestamp")]
    pub create_timestamp: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "UserTime100ns")]
    pub user_time100ns: u64,

    #[serde(rename = "KernelTime100ns")]
    pub kernel_time100ns: u64,

    #[serde(rename = "MemoryCommitBytes")]
    pub memory_commit_bytes: u64,

    #[serde(rename = "MemoryWorkingSetPrivateBytes")]
    pub memory_working_set_private_bytes: u64,

    #[serde(rename = "MemoryWorkingSetSharedBytes")]
    pub memory_working_set_shared_bytes: u64,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SharedMemoryRegionInfo {
    #[serde(rename = "SectionName")]
    pub section_name: String,

    #[serde(rename = "GuestPhysicalAddress")]
    pub guest_physical_address: u64,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GuestConnectionInfo {
    /// Each schema version x.y stands for the range of versions a.b where a==x
    /// and b<=y. This list comes from the SupportedSchemaVersions field in GcsCapabilities.
    #[serde(
        default,
        rename = "SupportedSchemaVersions",
        skip_serializing_if = "is_default"
    )]
    pub supported_schema_versions: Vec<schema::Version>,

    #[serde(
        default,
        rename = "ProtocolVersion",
        skip_serializing_if = "is_default"
    )]
    pub protocol_version: u32,

    #[serde(
        default,
        rename = "GuestDefinedCapabilities",
        skip_serializing_if = "serde_json::Value::is_null"
    )]
    pub guest_defined_capabilities: serde_json::Value,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CacheQueryStatsResponse {
    #[serde(rename = "L3OccupancyBytes")]
    pub l3_occupancy_bytes: u64,

    #[serde(rename = "L3TotalBwBytes")]
    pub l3_total_bw_bytes: u64,

    #[serde(rename = "L3LocalBwBytes")]
    pub l3_local_bw_bytes: u64,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Properties {
    #[serde(rename = "Id")]
    pub id: String,

    #[serde(rename = "SystemType")]
    pub system_type: SystemType,

    #[serde(default, rename = "RuntimeOsType", skip_serializing_if = "is_default")]
    pub runtime_os_type: OsType,

    #[serde(default, rename = "Name", skip_serializing_if = "is_default")]
    pub name: String,

    #[serde(default, rename = "Owner", skip_serializing_if = "is_default")]
    pub owner: String,

    #[serde(default, rename = "RuntimeId", skip_serializing_if = "is_default")]
    pub runtime_id: schema::utils::GuidSerde,

    #[serde(
        default,
        rename = "RuntimeTemplateId",
        skip_serializing_if = "is_default"
    )]
    pub runtime_template_id: String,

    #[serde(default, rename = "State", skip_serializing_if = "is_default")]
    pub state: State,

    #[serde(default, rename = "Stopped", skip_serializing_if = "is_default")]
    pub stopped: bool,

    #[serde(default, rename = "ExitType", skip_serializing_if = "is_default")]
    pub exit_type: NotificationType,

    #[serde(default, rename = "Memory", skip_serializing_if = "is_default")]
    pub memory: MemoryInformationForVm,

    #[serde(default, rename = "Statistics", skip_serializing_if = "is_default")]
    pub statistics: Statistics,

    #[serde(default, rename = "ProcessList", skip_serializing_if = "is_default")]
    pub process_list: Vec<ProcessDetails>,

    #[serde(
        default,
        rename = "TerminateOnLastHandleClosed",
        skip_serializing_if = "is_default"
    )]
    pub terminate_on_last_handle_closed: bool,

    #[serde(
        default,
        rename = "HostingSystemId",
        skip_serializing_if = "is_default"
    )]
    pub hosting_system_id: String,

    #[serde(
        default,
        rename = "SharedMemoryRegionInfo",
        skip_serializing_if = "is_default"
    )]
    pub shared_memory_region_info: Vec<SharedMemoryRegionInfo>,

    #[serde(
        default,
        rename = "GuestConnectionInfo",
        skip_serializing_if = "is_default"
    )]
    pub guest_connection_info: Option<GuestConnectionInfo>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SystemExitStatus {
    #[serde(rename = "Status")]
    pub status: i32,

    #[cfg(feature = "19h1")]
    #[serde(default, rename = "ExitType", skip_serializing_if = "is_default")]
    pub exit_type: NotificationType,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ProcessStatus {
    #[serde(rename = "ProcessId")]
    pub process_id: u32,

    #[serde(rename = "Exited")]
    pub exited: bool,

    #[serde(rename = "ExitCode")]
    pub exit_code: u32,

    #[serde(rename = "LastWaitResult")]
    pub last_wait_result: i32,
}

impl std::default::Default for WindowsCrashPhase {
    fn default() -> Self {
        WindowsCrashPhase::Inactive
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum WindowsCrashPhase {
    Inactive,
    CrashValues,
    Starting,
    Started,
    Writing,
    Complete,
}

/// Windows specific crash information
#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct WindowsCrashReport {
    #[serde(default, rename = "DumpFile", skip_serializing_if = "is_default")]
    pub dump_file: String,

    #[serde(default, rename = "OsMajorVersion", skip_serializing_if = "is_default")]
    pub os_major_version: u32,

    #[serde(default, rename = "OsMinorVersion", skip_serializing_if = "is_default")]
    pub os_minor_version: u32,

    #[serde(default, rename = "OsBuildNumber", skip_serializing_if = "is_default")]
    pub os_build_number: u32,

    #[serde(
        default,
        rename = "OsServicePackMajorVersion",
        skip_serializing_if = "is_default"
    )]
    pub os_service_pack_major_version: u32,

    #[serde(
        default,
        rename = "OsServicePackMinorVersion",
        skip_serializing_if = "is_default"
    )]
    pub os_service_pack_minor_version: u32,

    #[serde(default, rename = "OsSuiteMask", skip_serializing_if = "is_default")]
    pub os_suite_mask: u32,

    #[serde(default, rename = "OsProductType", skip_serializing_if = "is_default")]
    pub os_product_type: u32,

    #[serde(default, rename = "Status", skip_serializing_if = "is_default")]
    pub status: i32,

    #[serde(default, rename = "FinalPhase", skip_serializing_if = "is_default")]
    pub final_phase: WindowsCrashPhase,
}

/// crash information reported through CrashReport notifications
#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CrashReport {
    #[serde(rename = "SystemId")]
    pub system_id: String,

    #[serde(default, rename = "ActivityId", skip_serializing_if = "is_default")]
    pub activity_id: schema::utils::GuidSerde,

    #[serde(
        default,
        rename = "WindowsCrashInfo",
        skip_serializing_if = "is_default"
    )]
    pub windows_crash_info: Option<WindowsCrashReport>,

    #[serde(
        default,
        rename = "CrashParameters",
        skip_serializing_if = "is_default"
    )]
    pub crash_parameters: Vec<u64>,

    #[serde(default, rename = "CrashLog", skip_serializing_if = "is_default")]
    pub crash_log: String,
}
