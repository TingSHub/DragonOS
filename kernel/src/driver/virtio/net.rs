use virtio_drivers::{
    DeviceType, Transport, VirtIONet
};
use crate::include::bindings::bindings::{
    process_control_block, sched_enqueue, PROC_RUNNING, PROC_STOPPED,
};
use core::{sync::atomic::Ordering, str, format_args};
use crate::driver::uart::uart::{UartDriver, UartPort};
use self::virtio_impl::HalImpl;

mod virtio_impl;

fn virtio_device(transport: impl Transport) {
    match transport.device_type() {
        DeviceType::Network => virtio_net(transport),
        _ => UartDriver::uart_send(&UartPort::COM1, "Unrecognized virtio device"),
    }
}

fn virtio_net<T: Transport>(transport: T) {
    let mut net = VirtIONet::<HalImpl, T>::new(transport).expect("failed to create net driver");
    let mut buf = [0u8; 0x100];
    let len = net.recv(&mut buf).expect("failed to recv");
    let rec = str::from_utf8(&buf[..len]).expect("fail to convert str");
    UartDriver::uart_send(&UartPort::COM1, &rec);
    net.send(&buf[..len]).expect("failed to send");
    UartDriver::uart_send(&UartPort::COM1, "virtio-net test finished");
}