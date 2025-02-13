use embassy_stm32::{bind_interrupts, Peripheral, usart};
use embassy_stm32::mode::Async;
#[cfg(not(STM32C0))]
use embassy_stm32::peripherals::{DMA1_CH4, DMA1_CH5};
use embassy_stm32::peripherals::USART1;
use embassy_stm32::usart::{Config, ConfigError, RtsPin, RxPin, TxPin, Uart};
#[cfg(STM32C0)]
use embassy_stm32::usart::{RxDma, TxDma};
use crate::builder::uart::base::UartBase;
use crate::builder::uart::uart1::rx::{Uart1Rts, Uart1Rx, Uart1RxBuilder};
use crate::builder::uart::uart1::tx::{Uart1Cts, Uart1Tx};

pub mod rx;
pub mod tx;

bind_interrupts!(struct Irqs {
    USART1 => usart::InterruptHandler<USART1>;
});

/// uart1 builder
pub struct Uart1Builder {
    /// uart1 base data
    pub base: UartBase<USART1>,
    /// uart1 tx pin
    pub tx: Uart1Tx,
    /// uart1 rx pin
    pub rx: Uart1Rx,
    /// use rts cts
    pub rts_cts: Option<(Uart1Rts, Uart1Cts)>,
}

/// uart1 build
macro_rules! uart1_build {
    ($tx_dma:ty,$rx_dma:ty) => {
        /// build a serial port that supports read and write data
        pub fn build(self, tx_dma: $tx_dma, rx_dma: $rx_dma) -> Result<Uart<'static, Async>, ConfigError> {
            let rx = Uart1RxBuilder { base: self.base, rx: self.rx, rts: None };
            match self.tx {
                #[cfg(USART1_PA0)]
                Uart1Tx::PA0(pa0) => { Self::build_rx(pa0, rx, tx_dma, rx_dma, self.rts_cts) }
                #[cfg(PA9)]
                Uart1Tx::PA9(pa9) => { Self::build_rx(pa9, rx, tx_dma, rx_dma, self.rts_cts) }
                #[cfg(USART1_PB6)]
                Uart1Tx::PB6(pb6) => { Self::build_rx(pb6, rx, tx_dma, rx_dma, self.rts_cts) }
                #[cfg(USART1_PC14)]
                Uart1Tx::PC14(pc14) => { Self::build_rx(pc14, rx, tx_dma, rx_dma, self.rts_cts) }
            }
        }

        /// build by rx
        fn build_rx(
            tx: impl Peripheral<P=impl TxPin<USART1>> + 'static,
            rx: Uart1RxBuilder,
            tx_dma: $tx_dma,
            rx_dma: $rx_dma,
            rts_cts: Option<(Uart1Rts, Uart1Cts)>)
            -> Result<Uart<'static, Async>, ConfigError> {
            match rx.rx {
                #[cfg(USART1_PA1)]
                Uart1Rx::PA1(pa1) => { Self::build_rts(tx, pa1, rx.base, tx_dma, rx_dma, rts_cts) }
                #[cfg(USART1_RX_PA8)]
                Uart1Rx::PA8(pa8) => { Self::build_rts(tx, pa8, rx.base, tx_dma, rx_dma, rts_cts) }
                #[cfg(PA10)]
                Uart1Rx::PA10(pa10) => { Self::build_rts(tx, pa10, rx.base, tx_dma, rx_dma, rts_cts) }
                #[cfg(USART1_PB2)]
                Uart1Rx::PB2(pb2) => { Self::build_rts(tx, pb2, rx.base, tx_dma, rx_dma, rts_cts) }
                Uart1Rx::PB7(pb7) => { Self::build_rts(tx, pb7, rx.base, tx_dma, rx_dma, rts_cts) }
            }
        }

        /// build rts or default
        fn build_rts(
            tx: impl Peripheral<P=impl TxPin<USART1>> + 'static,
            rx: impl Peripheral<P=impl RxPin<USART1>> + 'static,
            base: UartBase<USART1>,
            tx_dma: $tx_dma,
            rx_dma: $rx_dma,
            rts_cts: Option<(Uart1Rts, Uart1Cts)>)
            -> Result<Uart<'static, Async>, ConfigError> {
            let (rts, cts) = crate::match_some_return!(rts_cts,
                Uart::new(base.uart, rx, tx, Irqs, tx_dma, rx_dma, base.config.unwrap_or_default()));
            match rts {
                #[cfg(USART1_PA12)]
                Uart1Rts::PA12(pa12) => { Self::build_cts(tx, rx, base, tx_dma, rx_dma, pa12, cts) }
                #[cfg(USART1_PA14)]
                Uart1Rts::PA14(pa14) => { Self::build_cts(tx, rx, base, tx_dma, rx_dma, pa14, cts) }
                #[cfg(USART1_PA15)]
                Uart1Rts::PA15(pa15) => { Self::build_cts(tx, rx, base, tx_dma, rx_dma, pa15, cts) }
                #[cfg(USART1_PB3)]
                Uart1Rts::PB3(pb3) => { Self::build_cts(tx, rx, base, tx_dma, rx_dma, pb3, cts) }
            }
        }

        fn build_cts(
            tx: impl Peripheral<P=impl TxPin<USART1>> + 'static,
            rx: impl Peripheral<P=impl RxPin<USART1>> + 'static,
            base: UartBase<USART1>,
            tx_dma: $tx_dma,
            rx_dma: $rx_dma,
            rts: impl Peripheral<P=impl RtsPin<USART1>> + 'static,
            cts: Uart1Cts,
        ) -> Result<Uart<'static, Async>, ConfigError> {
            match cts {
                #[cfg(USART1_PA11)]
                Uart1Cts::PA11(pa11) => { Uart::new_with_rtscts(base.uart, rx, tx, Irqs, rts, pa11, tx_dma, rx_dma, base.config.unwrap_or_default()) }
                #[cfg(USART1_PB4)]
                Uart1Cts::PB4(pb4) => { Uart::new_with_rtscts(base.uart, rx, tx, Irqs, rts, pb4, tx_dma, rx_dma, base.config.unwrap_or_default()) }
                #[cfg(USART1_CTS_PB6)]
                Uart1Cts::PB6(pb6) => { Uart::new_with_rtscts(base.uart, rx, tx, Irqs, rts, pb6, tx_dma, rx_dma, base.config.unwrap_or_default()) }
            }
        }
    };
}

/// custom method
impl Uart1Builder {
    /// create builder
    #[inline]
    pub fn new(uart: USART1, tx: Uart1Tx, rx: Uart1Rx) -> Self {
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
    pub fn rts_cts(mut self, rts: Uart1Rts, cts: Uart1Cts) -> Self {
        self.rts_cts = Some((rts, cts));
        self
    }

    #[cfg(STM32C0)]
    uart1_build!(impl Peripheral<P = impl TxDma<USART1>> + 'static,impl Peripheral<P = impl RxDma<USART1>> + 'static);
    #[cfg(not(STM32C0))]
    uart1_build!(DMA1_CH4,DMA1_CH5);
}
