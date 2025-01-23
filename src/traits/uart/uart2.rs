use embassy_stm32::peripherals::USART2;
use crate::builder::uart::uart2::rx::{Uart2Rx, Uart2RxBuilder};
use crate::builder::uart::uart2::tx::{Uart2Tx, Uart2TxBuilder};
use crate::builder::uart::uart2::Uart2Builder;

/// uart2 trait
pub trait Uart2Trait {
    /// create uart2 builder
    fn builder(self, tx: Uart2Tx, rx: Uart2Rx) -> Uart2Builder;

    /// create uart2 tx builder
    fn tx_builder(self, tx: Uart2Tx) -> Uart2TxBuilder;

    /// create uart2 rx builder
    fn rx_builder(self, rx: Uart2Rx) -> Uart2RxBuilder;
}

/// usart2 support uart2 trait
impl Uart2Trait for USART2 {
    #[inline]
    fn builder(self, tx: Uart2Tx, rx: Uart2Rx) -> Uart2Builder {
        Uart2Builder::new(self, tx, rx)
    }

    #[inline]
    fn tx_builder(self, tx: Uart2Tx) -> Uart2TxBuilder {
        Uart2TxBuilder::new(self, tx)
    }

    #[inline]
    fn rx_builder(self, rx: Uart2Rx) -> Uart2RxBuilder {
        Uart2RxBuilder::new(self, rx)
    }
}
