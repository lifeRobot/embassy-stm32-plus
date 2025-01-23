use embassy_stm32::peripherals::USART1;
use crate::builder::uart::uart1::rx::{Uart1Rx, Uart1RxBuilder};
use crate::builder::uart::uart1::tx::{Uart1Tx, Uart1TxBuilder};
use crate::builder::uart::uart1::Uart1Builder;

/// uart1 trait
pub trait Uart1Trait {
    /// create uart1 builder
    fn builder(self, tx: Uart1Tx, rx: Uart1Rx) -> Uart1Builder;

    /// create uart1 tx builder
    fn tx_builder(self,tx: Uart1Tx) -> Uart1TxBuilder;

    /// create uart1 rx builder
    fn rx_builder(self,rx: Uart1Rx) -> Uart1RxBuilder;
}

/// usart1 support uart1 trait
impl Uart1Trait for USART1 {
    #[inline]
    fn builder(self, tx: Uart1Tx, rx: Uart1Rx) -> Uart1Builder {
        Uart1Builder::new(self, tx, rx)
    }

    #[inline]
    fn tx_builder(self, tx: Uart1Tx) -> Uart1TxBuilder {
        Uart1TxBuilder::new(self, tx)
    }

    #[inline]
    fn rx_builder(self, rx: Uart1Rx) -> Uart1RxBuilder {
        Uart1RxBuilder::new(self, rx)
    }
}
