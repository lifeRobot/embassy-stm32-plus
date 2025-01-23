use embassy_stm32::peripherals::USART3;
use crate::builder::uart::uart3::rx::{Uart3Rx, Uart3RxBuilder};
use crate::builder::uart::uart3::tx::{Uart3Tx, Uart3TxBuilder};
use crate::builder::uart::uart3::Uart3Builder;

/// uart3 trait
pub trait Uart3Trait {
    /// create uart3 builder
    fn builder(self, tx: Uart3Tx, rx: Uart3Rx) -> Uart3Builder;

    /// create uart3 tx builder
    fn tx_builder(self, tx: Uart3Tx) -> Uart3TxBuilder;

    /// create uart3 rx builder
    fn rx_builder(self, rx: Uart3Rx) -> Uart3RxBuilder;
}

/// usart3 support uart3 trait
impl Uart3Trait for USART3 {
    #[inline]
    fn builder(self, tx: Uart3Tx, rx: Uart3Rx) -> Uart3Builder {
        Uart3Builder::new(self, tx, rx)
    }

    #[inline]
    fn tx_builder(self, tx: Uart3Tx) -> Uart3TxBuilder {
        Uart3TxBuilder::new(self, tx)
    }

    #[inline]
    fn rx_builder(self, rx: Uart3Rx) -> Uart3RxBuilder {
        Uart3RxBuilder::new(self, rx)
    }
}
