use embassy_stm32::eth::{PacketQueue, PHY};
use embassy_stm32::eth::generic_smi::GenericSMI;

/// eth phy
pub struct EthPhy<const TX: usize, const RX: usize, P: PHY> {
    /// queue
    pub queue: &'static mut PacketQueue<TX, RX>,
    /// phy
    pub phy: P,
    /// mac addr
    pub mac_addr: [u8; 6],
}

/// custom method
impl<const TX: usize, const RX: usize, P: PHY> EthPhy<TX, RX, P> {
    /// create eth phy
    #[inline]
    pub fn new(queue: &'static mut PacketQueue<TX, RX>, phy: P, mac_addr: [u8; 6]) -> Self {
        Self { queue, phy, mac_addr }
    }

    /// create eth pht from queue and phy
    #[inline]
    pub fn from_queue_phy(queue: &'static mut PacketQueue<TX, RX>, phy: P) -> Self {
        Self::new(queue, phy, [0x00, 0x00, 0xDE, 0xAD, 0xBE, 0xEF])
    }

    /// set mac addr
    #[inline]
    pub fn set_mac_addr(mut self, mac_addr: [u8; 6]) -> Self {
        self.mac_addr = mac_addr;
        self
    }
}

/// custom method
impl<const TX: usize, const RX: usize> EthPhy<TX, RX, GenericSMI> {
    /// create eth phy from queue
    #[inline]
    pub fn from_queue(queue: &'static mut PacketQueue<TX, RX>) -> Self {
        Self::new(queue, GenericSMI::new(0), [0x00, 0x00, 0xDE, 0xAD, 0xBE, 0xEF])
    }
}
