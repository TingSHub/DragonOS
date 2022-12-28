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



