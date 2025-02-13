use embassy_stm32::{bind_interrupts, Peripheral, usart};
use embassy_stm32::mode::Async;
#[cfg(not(STM32C0))]
use embassy_stm32::peripherals::{DMA1_CH6, DMA1_CH7};
use embassy_stm32::peripherals::USART2;
use embassy_stm32::usart::{Config, ConfigError, RtsPin, RxPin, TxPin, Uart};
#[cfg(STM32C0)]
use embassy_stm32::usart::{RxDma, TxDma};
use crate::builder::uart::base::UartBase;
use crate::builder::uart::uart2::rx::{Uart2Rts, Uart2Rx, Uart2RxBuilder};
use crate::builder::uart::uart2::tx::{Uart2Cts, Uart2Tx};

pub mod rx;
pub mod tx;

bind_interrupts!(struct Irqs {
    USART2 => usart::InterruptHandler<USART2>;
});

/// uart2 builder
pub struct Uart2Builder {
    /// uart2 base data
    pub base: UartBase<USART2>,
    /// uart2 tx pin
    pub tx: Uart2Tx,
    /// uart2 rx pin
    pub rx: Uart2Rx,
    /// use rts cts
    pub rts_cts: Option<(Uart2Rts, Uart2Cts)>,
}

/// uart2 build
macro_rules! uart2_build {
    ($tx_dma:ty,$rx_dma:ty) => {
        /// build a serial port that supports read and write data
        pub fn build(self, tx_dma: $tx_dma, rx_dma: $rx_dma) -> Result<Uart<'static, Async>, ConfigError> {
            let rx = Uart2RxBuilder { base: self.base, rx: self.rx, rts: None };
            match self.tx {
                Uart2Tx::PA2(pa2) => { Self::build_rx(pa2, rx, tx_dma, rx_dma, self.rts_cts) }
                #[cfg(USART2_TX_PA4)]
                Uart2Tx::PA4(pa4) => { Self::build_rx(pa4, rx, tx_dma, rx_dma, self.rts_cts) }
                #[cfg(USART2_PA8)]
                Uart2Tx::PA8(pa8) => { Self::build_rx(pa8, rx, tx_dma, rx_dma, self.rts_cts) }
                #[cfg(USART2_PA14)]
                Uart2Tx::PA14(pa14) => { Self::build_rx(pa14, rx, tx_dma, rx_dma, self.rts_cts) }
                #[cfg(USART2_PD5)]
                Uart2Tx::PD5(pd5) => { Self::build_rx(pd5, rx, tx_dma, rx_dma, self.rts_cts) }
            }
        }

        /// build by rx
        fn build_rx(
            tx: impl Peripheral<P=impl TxPin<USART2>> + 'static,
            rx: Uart2RxBuilder,
            tx_dma: $tx_dma,
            rx_dma: $rx_dma,
            rts_cts: Option<(Uart2Rts, Uart2Cts)>)
            -> Result<Uart<'static, Async>, ConfigError> {
            match rx.rx {
                #[cfg(USART2_PA3)]
                Uart2Rx::PA3(pa3) => { Self::build_rts(tx, pa3, rx.base, tx_dma, rx_dma, rts_cts) }
                #[cfg(USART2_PA5)]
                Uart2Rx::PA5(pa5) => { Self::build_rts(tx, pa5, rx.base, tx_dma, rx_dma, rts_cts) }
                #[cfg(USART2_PA13)]
                Uart2Rx::PA13(pa13) => { Self::build_rts(tx, pa13, rx.base, tx_dma, rx_dma, rts_cts) }
                #[cfg(USART2_PA14)]
                Uart2Rx::PA14(pa14) => { Self::build_rts(tx, pa14, rx.base, tx_dma, rx_dma, rts_cts) }
                #[cfg(USART2_PA15)]
                Uart2Rx::PA15(pa15) => { Self::build_rts(tx, pa15, rx.base, tx_dma, rx_dma, rts_cts) }
                #[cfg(USART2_PD6)]
                Uart2Rx::PD6(pd6) => { Self::build_rts(tx, pd6, rx.base, tx_dma, rx_dma, rts_cts) }
            }
        }

        /// build by rts
        fn build_rts(
            tx: impl Peripheral<P=impl TxPin<USART2>> + 'static,
            rx: impl Peripheral<P=impl RxPin<USART2>> + 'static,
            base: UartBase<USART2>,
            tx_dma: $tx_dma,
            rx_dma: $rx_dma,
            rts_cts: Option<(Uart2Rts, Uart2Cts)>)
            -> Result<Uart<'static, Async>, ConfigError> {
            let (rts, cts) = crate::match_some_return!(rts_cts,
                Uart::new(base.uart, rx, tx, Irqs, tx_dma, rx_dma, base.config.unwrap_or_default()));
            match rts {
                Uart2Rts::PA1(pa1) => { Self::build_cts(tx, rx, base, tx_dma, rx_dma, pa1, cts) }
                #[cfg(USART2_PB9)]
                Uart2Rts::PB9(pb9) => { Self::build_cts(tx, rx, base, tx_dma, rx_dma, pb9, cts) }
                #[cfg(USART2_PC14)]
                Uart2Rts::PC14(pc14) => { Self::build_cts(tx, rx, base, tx_dma, rx_dma, pc14, cts) }
                #[cfg(USART2_PD4)]
                Uart2Rts::PD4(pd4) => { Self::build_cts(tx, rx, base, tx_dma, rx_dma, pd4, cts) }
            }
        }

        /// build by cts
        fn build_cts(
            tx: impl Peripheral<P=impl TxPin<USART2>> + 'static,
            rx: impl Peripheral<P=impl RxPin<USART2>> + 'static,
            base: UartBase<USART2>,
            tx_dma: $tx_dma,
            rx_dma: $rx_dma,
            rts: impl Peripheral<P=impl RtsPin<USART2>> + 'static,
            cts: Uart2Cts)
            -> Result<Uart<'static, Async>, ConfigError> {
            match cts {
                Uart2Cts::PA0(pa0) => { Uart::new_with_rtscts(base.uart, rx, tx, Irqs, rts, pa0, tx_dma, rx_dma, base.config.unwrap_or_default()) }
                #[cfg(USART2_PB7)]
                Uart2Cts::PB7(pb7) => { Uart::new_with_rtscts(base.uart, rx, tx, Irqs, rts, pb7, tx_dma, rx_dma, base.config.unwrap_or_default()) }
                #[cfg(USART2_PB8)]
                Uart2Cts::PB8(pb8) => { Uart::new_with_rtscts(base.uart, rx, tx, Irqs, rts, pb8, tx_dma, rx_dma, base.config.unwrap_or_default()) }
                #[cfg(USART2_PD3)]
                Uart2Cts::PD3(pd3) => { Uart::new_with_rtscts(base.uart, rx, tx, Irqs, rts, pd3, tx_dma, rx_dma, base.config.unwrap_or_default()) }
            }
        }
    };
}

/// custom method
impl Uart2Builder {
    /// create builder
    #[inline]
    pub fn new(uart: USART2, tx: Uart2Tx, rx: Uart2Rx) -> Self {
        Self { base: UartBase::new(uart), tx, rx, rts_cts: None }
    }

    /// set uart config
    #[inline]
    pub fn config(mut self, config: Config) -> Self {
        self.base.set_config(config);
        self
    }

    /// set rts cts
    #[inline]
    pub fn rts_cts(mut self, rts: Uart2Rts, cts: Uart2Cts) -> Self {
        self.rts_cts = Some((rts, cts));
        self
    }

    #[cfg(STM32C0)]
    uart2_build!(impl Peripheral<P = impl TxDma<USART2>> + 'static,impl Peripheral<P = impl RxDma<USART2>> + 'static);
    #[cfg(not(STM32C0))]
    uart2_build!(DMA1_CH7,DMA1_CH6);
}
