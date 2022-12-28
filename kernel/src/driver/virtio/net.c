#include <common/kprint.h>
#include <common/errno.h>
#include <driver/pci/pci.h>
#include <debug/bug.h>
#include <common/spinlock.h>

#define MAX_NET_NUM 8 // pci总线上的virtio-net设备的最大数量

// 在pci总线上寻找到net设备控制器的header
static struct pci_device_structure_header_t *net_pdevs[MAX_NET_NUM];
static int net_pdevs_count = 0;

static uint8_t NETWORK_CLASS = 0x2;
static uint8_t ETHERNET_SUBCLASS = 0x0;

/**
 * @brief 
 *
 */
uint32_t get_virtio_net_mmio_addr()
{
    uint32_t target = 0;
    // 获取所有usb-pci设备的列表
    pci_get_device_structure(NETWORK_CLASS, ETHERNET_SUBCLASS, net_pdevs, &net_pdevs_count);

    if (WARN_ON(net_pdevs_count == 0)) {
        kwarn("There is no usb hardware in this computer!");
        return 0;
    }
    kdebug("net_pdevs_count=%d", net_pdevs_count);
    void* tmp = pci_read_header(0x0, net_pdevs[0]->bus, net_pdevs[0]->slot, net_pdevs[0]->func, )
    return target;
}
