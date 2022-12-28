use virtio_drivers::{
    DeviceType, Transport, VirtIONet, MmioTransport, VirtIOHeader
};
use core::{str, ptr::NonNull};
use fdt::{node::FdtNode, standard_nodes::Compatible, Fdt};
use crate::{
    include::bindings::bindings::{},
    kBUG, kdebug, kerror, kwarn
};
use self::virtio_impl::HalImpl;

mod virtio_impl;

const NETWORK_CLASS = 0x2;
const ETHERNET_SUBCLASS = 0x0;

fn get_net_pci_mmio() -> u32 {
    virtio_net_pci_dev: *mut *mut pci_device_structure_general_device_t;
    count: u32 = 0;
    unsafe {
        pci_get_device_structure(NETWORK_CLASS, ETHERNET_SUBCLASS, virtio_net_pci_devs, &count);
    }
}



