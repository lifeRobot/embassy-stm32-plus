use embassy_stm32::peripherals::{PA0, PA2, PD3, PD5, USART2};
use crate::builder::uart::base::UartBase;

/// uart2 tx pin
pub enum Uart2Tx {
    PA2(PA2),
    #[cfg(any(feature = "pin_100", feature = "pin_144"))]
    PD5(PD5),
}

/// uart2 cts pin
pub enum Uart2Cts {
    PA0(PA0),
    #[cfg(any(feature = "pin_100", feature = "pin_144"))]
    PD3(PD3),
}

/// uart2 tx builder
pub struct Uart2TxBuilder {
    /// uart2 base data
    pub base: UartBase<USART2>,
    /// tx pin
    pub tx: Uart2Tx,
    /// use cts
    pub cts: Option<Uart2Cts>,
}

/// custom method
impl Uart2TxBuilder {
    /// create builder
    #[inline]
    pub fn new(uart: USART2, tx: Uart2Tx) -> Self {
        Self { base: UartBase::new(uart), tx, cts: None }
    }
}
