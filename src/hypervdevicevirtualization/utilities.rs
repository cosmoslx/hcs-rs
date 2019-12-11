// Copyright © rafawo (rafawo1@hotmail.com). All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

//! Rust types that provide convenient functionality built on top of the hypervdevicevirtualiation APIs.

use crate::hypervdevicevirtualization;
use crate::hypervdevicevirtualization::defs::*;
use crate::HcsResult;
use winutils_rs::windefs::*;

pub trait HdvPciDevice {
    fn initialize(&mut self) -> HResult;
    fn teardown(&mut self);
    fn set_configuration(&mut self, values: &[PCWStr]) -> HResult;
    fn get_details(&self, pnpi_id: PHdvPciPnpId, probed_bars: &mut [u32]) -> HResult;
    fn start(&mut self) -> HResult;
    fn stop(&mut self);
    fn read_config_space(&self, offset: u32, value: &mut u32) -> HResult;
    fn write_config_space(&mut self, offset: u32, value: u32) -> HResult;
    fn read_intercepted_memory(
        &self,
        bar_index: HdvPciBarSelector,
        offset: u64,
        value: &mut [Byte],
    ) -> HResult;
    fn write_intercepted_memory(
        &mut self,
        bar_index: HdvPciBarSelector,
        offset: u64,
        value: &[Byte],
    ) -> HResult;
}

/// Wrapper object that abstracts out setting up the C-style callbacks into
/// the hypervdevicevirtualization framework. When such callbacks are fired,
/// it will redirect the function call to the stored `device` trait object.
pub struct HdvPciDeviceBase {
    pub device: Box<dyn HdvPciDevice>,
    device_handle: HdvDeviceHandle,
}

impl HdvPciDeviceBase {
    /// Creates a new `HdvPciDeviceBase` object that abstracts out setting up
    /// the C-style callbacks into the hyperdevicevirtualization framework.
    /// It will store the supplied `device` internally, taking ownership of it.
    /// When the actual C callback is fired, it will redirect the function call
    /// to the stored `device` trait object.
    pub fn new(
        device_host_handle: HdvHostHandle,
        device_class_id: &Guid,
        device_instance_id: &Guid,
        device: Box<dyn HdvPciDevice>,
    ) -> HcsResult<HdvPciDeviceBase> {
        let mut device_base = HdvPciDeviceBase {
            device,
            device_handle: std::ptr::null_mut(),
        };
        device_base.device_handle = hypervdevicevirtualization::create_device_instance(
            device_host_handle,
            HdvDeviceType::Pci,
            device_class_id,
            device_instance_id,
            &device_base_interface::DEVICE_INTERFACE as *const _ as *const Void,
            &mut device_base as *mut _ as PVoid,
        )?;
        Ok(device_base)
    }

    /// Writes the contents of the supplied buffer to guest primary memory (RAM).
    pub fn write_guest_memory_buffer(
        &self,
        guest_physical_address: u64,
        buffer: &[Byte],
    ) -> HcsResult<()> {
        hypervdevicevirtualization::write_guest_memory(
            self.device_handle,
            guest_physical_address,
            buffer,
        )
    }

    /// Writes the supplied object as a byte buffer to guest primary memory (RAM).
    pub fn write_guest_memory<T>(&self, guest_physical_address: u64, data: &T) -> HcsResult<()>
    where
        T: Sized,
    {
        self.write_guest_memory_buffer(guest_physical_address, unsafe {
            std::slice::from_raw_parts(data as *const _ as *const u8, std::mem::size_of::<T>())
        })
    }
}

pub(crate) mod device_base_interface {
    use super::*;

    unsafe extern "system" fn initialize(device_context: *mut Void) -> HResult {
        let device_base: *mut HdvPciDeviceBase = device_context as *mut HdvPciDeviceBase;
        (*device_base).device.initialize()
    }

    unsafe extern "system" fn teardown(device_context: *mut Void) {
        let device_base: *mut HdvPciDeviceBase = device_context as *mut HdvPciDeviceBase;
        (*device_base).device.teardown();
    }

    unsafe extern "system" fn set_configuration(
        device_context: *mut Void,
        configuration_value_count: u32,
        configuration_values: *const PCWStr,
    ) -> HResult {
        let device_base: *mut HdvPciDeviceBase = device_context as *mut HdvPciDeviceBase;
        let config_values: &[PCWStr] =
            std::slice::from_raw_parts(configuration_values, configuration_value_count as usize);
        (*device_base).device.set_configuration(config_values)
    }

    unsafe extern "system" fn get_details(
        device_context: *mut Void,
        pnp_id: PHdvPciPnpId,
        probed_bars_count: u32,
        probed_bars: *mut u32,
    ) -> HResult {
        let device_base: *const HdvPciDeviceBase = device_context as *const HdvPciDeviceBase;
        let probed_bars: &mut [u32] =
            std::slice::from_raw_parts_mut(probed_bars, probed_bars_count as usize);
        (*device_base).device.get_details(pnp_id, probed_bars)
    }

    unsafe extern "system" fn start(device_context: *mut Void) -> HResult {
        let device_base: *mut HdvPciDeviceBase = device_context as *mut HdvPciDeviceBase;
        (*device_base).device.start()
    }

    unsafe extern "system" fn stop(device_context: *mut Void) {
        let device_base: *mut HdvPciDeviceBase = device_context as *mut HdvPciDeviceBase;
        (*device_base).device.stop();
    }

    unsafe extern "system" fn read_config_space(
        device_context: *mut Void,
        offset: u32,
        value: *mut u32,
    ) -> HResult {
        let device_base: *const HdvPciDeviceBase = device_context as *const HdvPciDeviceBase;
        (*device_base).device.read_config_space(offset, &mut *value)
    }

    unsafe extern "system" fn write_config_space(
        device_context: *mut Void,
        offset: u32,
        value: u32,
    ) -> HResult {
        let device_base: *mut HdvPciDeviceBase = device_context as *mut HdvPciDeviceBase;
        (*device_base).device.write_config_space(offset, value)
    }

    unsafe extern "system" fn read_intercepted_memory(
        device_context: *mut Void,
        bar_index: HdvPciBarSelector,
        offset: u64,
        length: u64,
        value: *mut Byte,
    ) -> HResult {
        let device_base: *const HdvPciDeviceBase = device_context as *const HdvPciDeviceBase;
        let values: &mut [Byte] = std::slice::from_raw_parts_mut(value, length as usize);
        (*device_base)
            .device
            .read_intercepted_memory(bar_index, offset, values)
    }

    unsafe extern "system" fn write_intercepted_memory(
        device_context: *mut Void,
        bar_index: HdvPciBarSelector,
        offset: u64,
        length: u64,
        value: *const Byte,
    ) -> HResult {
        let device_base: *mut HdvPciDeviceBase = device_context as *mut HdvPciDeviceBase;
        let values: &[Byte] = std::slice::from_raw_parts(value, length as usize);
        (*device_base)
            .device
            .write_intercepted_memory(bar_index, offset, values)
    }

    /// Global HDV PCI device interface object with all the necessary
    /// callbacks assigned to global unsafe functions that take care
    /// of forwarding the calls to higher-abstract structs.
    pub static DEVICE_INTERFACE: HdvPciDeviceInterface = HdvPciDeviceInterface {
        version: HdvPciInterfaceVersion::Version1,
        initialize: Some(initialize),
        teardown: Some(teardown),
        set_configuration: Some(set_configuration),
        get_details: Some(get_details),
        start: Some(start),
        stop: Some(stop),
        read_config_space: Some(read_config_space),
        write_config_space: Some(write_config_space),
        read_intercepted_memory: Some(read_intercepted_memory),
        write_intercepted_memory: Some(write_intercepted_memory),
    };
}
