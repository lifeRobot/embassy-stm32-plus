use embassy_stm32::peripherals::{PC12, PD2, UART5};
use crate::builder::uart::uart5::rx::Uart5RxBuilder;
use crate::builder::uart::uart5::tx::Uart5TxBuilder;
use crate::builder::uart::uart5::Uart5Builder;

/// uart5 trait
pub trait Uart5Trait {
    /// create uart5 builder
    #[deprecated(note = "no any dma support TxDma<UART5> or RxDma<UART5>")]
    fn builder(self, tx: PC12, rx: PD2) -> Uart5Builder;

    /// create uart5 tx builder
    #[deprecated(note = "no any dma support TxDma<UART5>")]
    fn tx_builder(self, tx: PC12) -> Uart5TxBuilder;

    /// create uart4 rx builder
    #[deprecated(note = "no any dma support RxDma<UART5>")]
    fn rx_builder(self, rx: PD2) -> Uart5RxBuilder;
}

/// uart4 support uart3 trait
impl Uart5Trait for UART5 {
    #[inline]
    fn builder(self, tx: PC12, rx: PD2) -> Uart5Builder {
        #[allow(deprecated)]
        Uart5Builder::new(self, tx, rx)
    }

    #[inline]
    fn tx_builder(self, tx: PC12) -> Uart5TxBuilder {
        #[allow(deprecated)]
        Uart5TxBuilder::new(self, tx)
    }

    #[inline]
    fn rx_builder(self, rx: PD2) -> Uart5RxBuilder {
        #[allow(deprecated)]
        Uart5RxBuilder::new(self, rx)
    }
}
