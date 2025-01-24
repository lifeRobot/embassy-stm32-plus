use embassy_stm32::peripherals::{PA1, PA2, PA7, PB11, PB12, PB13, PC1, PC4, PC5};
#[cfg(PD10)]
use embassy_stm32::peripherals::PD10;
#[cfg(PD8)]
use embassy_stm32::peripherals::PD8;
#[cfg(PD9)]
use embassy_stm32::peripherals::PD9;

/// eth crs pin
pub enum EthCrs {
    PA7(PA7),
    #[cfg(PD8)]
    PD8(PD8),
}

/// eth rx_d0 pin
pub enum EthRxD0 {
    PC4(PC4),
    #[cfg(PD9)]
    PD9(PD9),
}

/// eth rx_d1 pin
pub enum EthRxD1 {
    PC5(PC5),
    #[cfg(PD10)]
    PD10(PD10),
}

/// eth pin once
pub(crate) struct EthPinOnce {
    /// ref_clk pin
    pub ref_clk: PA1,
    /// mdio pin
    pub mdio: PA2,
    /// mdc pin
    pub mdc: PC1,
    /// tx_d0 pin
    pub tx_d0: PB12,
    /// tx_d1 pin
    pub tx_d1: PB13,
    /// tx_en pin
    pub tx_en: PB11,
}

/// eth pins
pub struct EthPins {
    /// ref_clk pin
    pub ref_clk: PA1,
    /// mdio pin
    pub mdio: PA2,
    /// mdc pin
    pub mdc: PC1,
    /// crs pin
    pub crs: EthCrs,
    /// rx_d0 pin
    pub rx_d0: EthRxD0,
    /// rx_d1 pin
    pub rx_d1: EthRxD1,
    /// tx_d0 pin
    pub tx_d0: PB12,
    /// tx_d1 pin
    pub tx_d1: PB13,
    /// tx_en pin
    pub tx_en: PB11,
}

/// custom method
impl EthPins {
    /// split pins
    pub(crate) fn split(self) -> (EthCrs, EthRxD0, EthRxD1, EthPinOnce) {
        (self.crs, self.rx_d0, self.rx_d1, EthPinOnce {
            ref_clk: self.ref_clk,
            mdio: self.mdio,
            mdc: self.mdc,
            tx_d0: self.tx_d0,
            tx_d1: self.tx_d1,
            tx_en: self.tx_en,
        })
    }
}
