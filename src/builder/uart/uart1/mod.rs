use embassy_stm32::{bind_interrupts, Peripheral, usart};
use embassy_stm32::mode::Async;
#[cfg(not(STM32C0))]
use embassy_stm32::peripherals::{DMA1_CH4, DMA1_CH5};
#[cfg(STM32C0)]
use embassy_stm32::peripherals::{DMA1_CH1, DMA1_CH2};
use embassy_stm32::peripherals::{PA11, PA12, USART1};
use embassy_stm32::usart::{Config, ConfigError, RxPin, TxPin, Uart};
use crate::builder::uart::base::UartBase;
use crate::builder::uart::uart1::rx::{Uart1Rx, Uart1RxBuilder};
use crate::builder::uart::uart1::tx::Uart1Tx;

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
    pub rts_cts: Option<(PA12, PA11)>,
}

/// uart1 build
macro_rules! uart1_build {
    ($tx_dma:ty,$rx_dma:ty) => {
        /// build a serial port that supports read and write data
        pub fn build(self, tx_dma: $tx_dma, rx_dma: $rx_dma) -> Result<Uart<'static, Async>, ConfigError> {
            let rx = Uart1RxBuilder { base: self.base, rx: self.rx, rts: None };
            match self.tx {
                Uart1Tx::PA9(pa9) => { Self::build_rx(pa9, rx, tx_dma, rx_dma, self.rts_cts) }
                Uart1Tx::PB6(pb6) => { Self::build_rx(pb6, rx, tx_dma, rx_dma, self.rts_cts) }
            }
        }

        /// build by rx
        fn build_rx(
            tx: impl Peripheral<P=impl TxPin<USART1>> + 'static,
            rx: Uart1RxBuilder,
            tx_dma: $tx_dma,
            rx_dma: $rx_dma,
            rts_cts: Option<(PA12, PA11)>)
            -> Result<Uart<'static, Async>, ConfigError> {
            match rx.rx {
                Uart1Rx::PA10(pa10) => { Self::build_rts_cts(tx, pa10, rx.base, tx_dma, rx_dma, rts_cts) }
                Uart1Rx::PB7(pb7) => { Self::build_rts_cts(tx, pb7, rx.base, tx_dma, rx_dma, rts_cts) }
            }
        }

        /// build rts_cts or default
        fn build_rts_cts(
            tx: impl Peripheral<P=impl TxPin<USART1>> + 'static,
            rx: impl Peripheral<P=impl RxPin<USART1>> + 'static,
            base: UartBase<USART1>,
            tx_dma: $tx_dma,
            rx_dma: $rx_dma,
            rts_cts: Option<(PA12, PA11)>)
            -> Result<Uart<'static, Async>, ConfigError> {
            let (rts, cts) = crate::match_some_return!(rts_cts,
                Uart::new(base.uart, rx, tx, Irqs, tx_dma, rx_dma, base.config.unwrap_or_default()));
            Uart::new_with_rtscts(base.uart, rx, tx, Irqs, rts, cts, tx_dma, rx_dma, base.config.unwrap_or_default())
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
    pub fn rts_cts(mut self, rts: PA12, cts: PA11) -> Self {
        self.rts_cts = Some((rts, cts));
        self
    }

    #[cfg(STM32C0)]
    uart1_build!(DMA1_CH2,DMA1_CH1);
    #[cfg(not(STM32C0))]
    uart1_build!(DMA1_CH4,DMA1_CH5);
    /*/// build a serial port that supports read and write data
    pub fn build(self, tx_dma: DMA1_CH4, rx_dma: DMA1_CH5) -> Result<Uart<'static, Async>, ConfigError> {
        let rx = Uart1RxBuilder { base: self.base, rx: self.rx, rts: None };
        match self.tx {
            Uart1Tx::PA9(pa9) => { Self::build_rx(pa9, rx, tx_dma, rx_dma, self.rts_cts) }
            Uart1Tx::PB6(pb6) => { Self::build_rx(pb6, rx, tx_dma, rx_dma, self.rts_cts) }
        }
    }

    /// build by rx
    fn build_rx(
        tx: impl Peripheral<P=impl TxPin<USART1>> + 'static,
        rx: Uart1RxBuilder,
        tx_dma: DMA1_CH4,
        rx_dma: DMA1_CH5,
        rts_cts: Option<(PA12, PA11)>)
        -> Result<Uart<'static, Async>, ConfigError> {
        match rx.rx {
            Uart1Rx::PA10(pa10) => { Self::build_rts_cts(tx, pa10, rx.base, tx_dma, rx_dma, rts_cts) }
            Uart1Rx::PB7(pb7) => { Self::build_rts_cts(tx, pb7, rx.base, tx_dma, rx_dma, rts_cts) }
        }
    }

    /// build rts_cts or default
    fn build_rts_cts(
        tx: impl Peripheral<P=impl TxPin<USART1>> + 'static,
        rx: impl Peripheral<P=impl RxPin<USART1>> + 'static,
        base: UartBase<USART1>,
        tx_dma: DMA1_CH4,
        rx_dma: DMA1_CH5,
        rts_cts: Option<(PA12, PA11)>)
        -> Result<Uart<'static, Async>, ConfigError> {
        let (rts, cts) = crate::match_some_return!(rts_cts,
            Uart::new(base.uart, rx, tx, Irqs, tx_dma, rx_dma, base.config.unwrap_or_default()));
        Uart::new_with_rtscts(base.uart, rx, tx, Irqs, rts, cts, tx_dma, rx_dma, base.config.unwrap_or_default())
    }*/
}
