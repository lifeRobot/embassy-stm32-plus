use embassy_stm32::peripherals::{PC10, PC11, UART4};
use crate::builder::uart::uart4::rx::Uart4RxBuilder;
use crate::builder::uart::uart4::tx::Uart4TxBuilder;
use crate::builder::uart::uart4::Uart4Builder;

/// uart4 trait
pub trait Uart4Trait {
    /// create uart4 builder
    fn builder(self, tx: PC10, rx: PC11) -> Uart4Builder;

    /// create uart4 tx builder
    fn tx_builder(self, tx: PC10) -> Uart4TxBuilder;

    /// create uart4 rx builder
    fn rx_builder(self, rx: PC11) -> Uart4RxBuilder;
}

/// uart4 support uart3 trait
impl Uart4Trait for UART4 {
    #[inline]
    fn builder(self, tx: PC10, rx: PC11) -> Uart4Builder {
        Uart4Builder::new(self, tx, rx)
    }

    #[inline]
    fn tx_builder(self, tx: PC10) -> Uart4TxBuilder {
        Uart4TxBuilder::new(self, tx)
    }

    #[inline]
    fn rx_builder(self, rx: PC11) -> Uart4RxBuilder {
        Uart4RxBuilder::new(self, rx)
    }
}
