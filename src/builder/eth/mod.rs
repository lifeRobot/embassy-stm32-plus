use embassy_stm32::{bind_interrupts, eth, Peripheral};
use embassy_stm32::eth::{CRSPin, Ethernet, RXD0Pin, PHY};
use embassy_stm32::peripherals::ETH;
use crate::builder::eth::pins::{EthCrs, EthRxD0, EthRxD1};

pub mod pins;
pub mod phy;

bind_interrupts!(struct Irqs {
    ETH => eth::InterruptHandler;
});

/// eth once
struct EthOnce<const TX: usize, const RX: usize, P: PHY> {
    /// eth device
    pub eth: ETH,
    /// eth pin once
    pub pin_once: pins::EthPinOnce,
    /// eth phy
    pub phy: phy::EthPhy<TX, RX, P>,
}

/// eth builder
pub struct EthBuilder<const TX: usize, const RX: usize, P: PHY> {
    /// eth device
    pub eth: ETH,
    /// eth pins
    pub pins: pins::EthPins,
    /// eth phy
    pub phy: phy::EthPhy<TX, RX, P>,
}

/// custom method
impl<const TX: usize, const RX: usize, P: PHY> EthBuilder<TX, RX, P> {
    /// create builder
    #[inline]
    pub fn new(eth: ETH, pins: pins::EthPins, phy: phy::EthPhy<TX, RX, P>) -> Self {
        Self { eth, pins, phy }
    }

    /// build ethernet, more see [Ethernet::new]
    pub fn build(self) -> Ethernet<'static, ETH, P> {
        let (crs, rx_d0, rx_d1, eth_once) = self.split();
        match crs {
            EthCrs::PA7(pa7) => { Self::build_rx_d0(pa7, rx_d0, rx_d1, eth_once) }
            #[cfg(PD8)]
            EthCrs::PD8(pa8) => { Self::build_rx_d0(pa8, rx_d0, rx_d1, eth_once) }
        }
    }

    /// build by rx_d0
    fn build_rx_d0(crs: impl Peripheral<P=impl CRSPin<ETH>> + 'static, rx_d0: EthRxD0, rx_d1: EthRxD1, eth_once: EthOnce<TX, RX, P>)
                   -> Ethernet<'static, ETH, P> {
        match rx_d0 {
            EthRxD0::PC4(pc4) => { Self::build_rx_d1(crs, pc4, rx_d1, eth_once) }
            #[cfg(PD9)]
            EthRxD0::PD9(pd9) => { Self::build_rx_d1(crs, pd9, rx_d1, eth_once) }
        }
    }

    /// build by rx_d1
    fn build_rx_d1(
        crs: impl Peripheral<P=impl CRSPin<ETH>> + 'static,
        rx_d0: impl Peripheral<P=impl RXD0Pin<ETH>> + 'static,
        rx_d1: EthRxD1,
        eth_once: EthOnce<TX, RX, P>,
    ) -> Ethernet<'static, ETH, P> {
        let EthOnce { eth, pin_once, phy } = eth_once;
        match rx_d1 {
            EthRxD1::PC5(pc5) => { Ethernet::new(phy.queue, eth, Irqs, pin_once.ref_clk, pin_once.mdio, pin_once.mdc, crs, rx_d0, pc5, pin_once.tx_d0, pin_once.tx_d1, pin_once.tx_en, phy.phy, phy.mac_addr) }
            #[cfg(PD10)]
            EthRxD1::PD10(pd10) => { Ethernet::new(phy.queue, eth, Irqs, pin_once.ref_clk, pin_once.mdio, pin_once.mdc, crs, rx_d0, pd10, pin_once.tx_d0, pin_once.tx_d1, pin_once.tx_en, phy.phy, phy.mac_addr) }
        }
    }

    /// split eth
    fn split(self) -> (EthCrs, EthRxD0, EthRxD1, EthOnce<TX, RX, P>) {
        let (crs, rx_d0, rx_d1, pin_once) = self.pins.split();
        (crs, rx_d0, rx_d1, EthOnce {
            eth: self.eth,
            pin_once,
            phy: self.phy,
        })
    }
}
