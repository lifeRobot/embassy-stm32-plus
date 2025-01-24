use embassy_stm32::eth::PHY;
use embassy_stm32::peripherals::ETH;
use crate::builder::eth::{phy, pins, EthBuilder};
use crate::builder::eth::phy::EthPhy;
use crate::builder::eth::pins::EthPins;

/// eth trait
pub trait EthTrait<const TX: usize, const RX: usize, P: PHY> {
    /// create eth builder
    fn builder(self, pins: pins::EthPins, phy: phy::EthPhy<TX, RX, P>) -> EthBuilder<TX, RX, P>;
}

/// eth support eth trait
impl<const TX: usize, const RX: usize, P: PHY> EthTrait<TX, RX, P> for ETH {
    #[inline]
    fn builder(self, pins: EthPins, phy: EthPhy<TX, RX, P>) -> EthBuilder<TX, RX, P> {
        EthBuilder::new(self, pins, phy)
    }
}
