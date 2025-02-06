use embassy_stm32::mode::Async;
use embassy_stm32::Peripheral;
#[cfg(not(STM32C0))]
use embassy_stm32::peripherals::DMA1_CH4;
#[cfg(STM32C0)]
use embassy_stm32::peripherals::DMA1_CH2;
use embassy_stm32::peripherals::{PA11, PA9, PB6, USART1};
use embassy_stm32::usart::{Config, ConfigError, TxPin, UartTx};
use crate::builder::uart::base::UartBase;

/// uart1 tx pin
pub enum Uart1Tx {
    PA9(PA9),
    PB6(PB6),
}

/// uart1 tx builder
pub struct Uart1TxBuilder {
    /// uart1 base data
    pub base: UartBase<USART1>,
    /// tx pin
    pub tx: Uart1Tx,
    /// use cts
    pub cts: Option<PA11>,
}

/// uart1 tx build
macro_rules! uart1_tx_build {
    ($tx_dma:ty) => {
        /// build uart tx that supports write data
        pub fn build(self, tx_dma: $tx_dma) -> Result<UartTx<'static, Async>, ConfigError> {
            match self.tx {
                Uart1Tx::PA9(pa9) => { Self::build_cts(pa9, tx_dma, self.base, self.cts) }
                Uart1Tx::PB6(pb6) => { Self::build_cts(pb6, tx_dma, self.base, self.cts) }
            }
        }

        /// build cts or default
        fn build_cts(
            tx: impl Peripheral<P=impl TxPin<USART1>> + 'static,
            tx_dma: $tx_dma,
            base: UartBase<USART1>,
            cts: Option<PA11>)
            -> Result<UartTx<'static, Async>, ConfigError> {
            let cts = crate::match_some_return!(cts,
                UartTx::new(base.uart, tx, tx_dma, base.config.unwrap_or_default()));
            UartTx::new_with_cts(base.uart, tx, cts, tx_dma, base.config.unwrap_or_default())
        }
    };
}

/// custom method
impl Uart1TxBuilder {
    /// create builder<br />
    /// default baud_rate=115200,data_bits=8,stop_bits=1,parity=None<br />
    /// set config see [Self::config], default config see [Config::default]
    #[inline]
    pub fn new(uart: USART1, tx: Uart1Tx) -> Self {
        Self { base: UartBase::new(uart), tx, cts: None }
    }

    /// set uart config
    #[inline]
    pub fn config(mut self, config: Config) -> Self {
        self.base.set_config(config);
        self
    }

    /// set cts pin
    #[inline]
    pub fn cts(mut self, cts: PA11) -> Self {
        self.cts = Some(cts);
        self
    }

    #[cfg(STM32C0)]
    uart1_tx_build!(DMA1_CH2);
    #[cfg(not(STM32C0))]
    uart1_tx_build!(DMA1_CH4);
    /*/// build uart tx that supports write data
    pub fn build(self, tx_dma: DMA1_CH4) -> Result<UartTx<'static, Async>, ConfigError> {
        match self.tx {
            Uart1Tx::PA9(pa9) => { Self::build_cts(pa9, tx_dma, self.base, self.cts) }
            Uart1Tx::PB6(pb6) => { Self::build_cts(pb6, tx_dma, self.base, self.cts) }
        }
    }

    /// build cts or default
    fn build_cts(
        tx: impl Peripheral<P=impl TxPin<USART1>> + 'static,
        tx_dma: DMA1_CH4,
        base: UartBase<USART1>,
        cts: Option<PA11>)
        -> Result<UartTx<'static, Async>, ConfigError> {
        let cts = crate::match_some_return!(cts,
            UartTx::new(base.uart, tx, tx_dma, base.config.unwrap_or_default()));
        UartTx::new_with_cts(base.uart, tx, cts, tx_dma, base.config.unwrap_or_default())
    }*/
}
