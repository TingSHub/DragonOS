use virtio_drivers::{
    DeviceType, Transport, VirtIONet, Hal, PhysAddr, VirtAddr
};

pub struct HalImpl;

impl Hal for HalImpl {
    fn dma_alloc(pages: usize) -> PhysAddr {
        0
    }
    fn dma_dealloc(paddr: PhysAddr, pages: usize) -> i32 {
        //trace!("dealloc DMA: paddr={:#x}, pages={}", paddr, pages);
        0
    }
    fn phys_to_virt(paddr: PhysAddr) -> VirtAddr {
        paddr
    }
    fn virt_to_phys(vaddr: VirtAddr) -> PhysAddr {
        vaddr
    }
}