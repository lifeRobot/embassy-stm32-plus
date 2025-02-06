use crate::builder::uart::base::UartBase;
use embassy_stm32::mode::Async;
#[cfg(not(STM32C0))]
use embassy_stm32::peripherals::DMA1_CH7;
#[cfg(STM32C0)]
use embassy_stm32::peripherals::DMA1_CH1;
use embassy_stm32::peripherals::{PA0, PA2, USART2};
#[cfg(PD3)]
use embassy_stm32::peripherals::PD3;
#[cfg(PD5)]
use embassy_stm32::peripherals::PD5;
use embassy_stm32::usart::{Config, ConfigError, TxPin, UartTx};
use embassy_stm32::Peripheral;

/// uart2 tx pin
pub enum Uart2Tx {
    PA2(PA2),
    #[cfg(PD5)]
    PD5(PD5),
}

/// uart2 cts pin
pub enum Uart2Cts {
    PA0(PA0),
    #[cfg(PD3)]
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

/// uart2 tx build
macro_rules! uart2_tx_build {
    ($tx_dma:ty) => {
        /// build uart tx that supports write data
        pub fn build(self, tx_dma: $tx_dma) -> Result<UartTx<'static, Async>, ConfigError> {
            match self.tx {
                Uart2Tx::PA2(pa2) => Self::build_cts(pa2, tx_dma, self.base, self.cts),
                #[cfg(PD5)]
                Uart2Tx::PD5(pa5) => Self::build_cts(pa5, tx_dma, self.base, self.cts),
            }
        }

        /// build by cts
        fn build_cts(
            tx: impl Peripheral<P=impl TxPin<USART2>> + 'static,
            tx_dma: $tx_dma,
            base: UartBase<USART2>,
            cts: Option<Uart2Cts>)
            -> Result<UartTx<'static, Async>, ConfigError> {
            let cts = crate::match_some_return!(cts,
                UartTx::new(base.uart, tx, tx_dma, base.config.unwrap_or_default()));

            match cts {
                Uart2Cts::PA0(pa0) => { UartTx::new_with_cts(base.uart, tx, pa0, tx_dma, base.config.unwrap_or_default()) }
                #[cfg(PD3)]
                Uart2Cts::PD3(pd3) => { UartTx::new_with_cts(base.uart, tx, pd3, tx_dma, base.config.unwrap_or_default()) }
            }
        }
    };
}

/// custom method
impl Uart2TxBuilder {
    /// create builder
    #[inline]
    pub fn new(uart: USART2, tx: Uart2Tx) -> Self {
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
    pub fn cts(mut self, cts: Uart2Cts) -> Self {
        self.cts = Some(cts);
        self
    }

    #[cfg(STM32C0)]
    uart2_tx_build!(DMA1_CH1);
    #[cfg(not(STM32C0))]
    uart2_tx_build!(DMA1_CH7);
    /*/// build uart tx that supports write data
    pub fn build(self, tx_dma: DMA1_CH7) -> Result<UartTx<'static, Async>, ConfigError> {
        match self.tx {
            Uart2Tx::PA2(pa2) => Self::build_cts(pa2, tx_dma, self.base, self.cts),
            #[cfg(PD5)]
            Uart2Tx::PD5(pa5) => Self::build_cts(pa5, tx_dma, self.base, self.cts),
        }
    }

    /// build by cts
    fn build_cts(
        tx: impl Peripheral<P=impl TxPin<USART2>> + 'static,
        tx_dma: DMA1_CH7,
        base: UartBase<USART2>,
        cts: Option<Uart2Cts>)
        -> Result<UartTx<'static, Async>, ConfigError> {
        let cts = crate::match_some_return!(cts,
            UartTx::new(base.uart, tx, tx_dma, base.config.unwrap_or_default()));

        match cts {
            Uart2Cts::PA0(pa0) => { UartTx::new_with_cts(base.uart, tx, pa0, tx_dma, base.config.unwrap_or_default()) }
            #[cfg(PD3)]
            Uart2Cts::PD3(pd3) => { UartTx::new_with_cts(base.uart, tx, pd3, tx_dma, base.config.unwrap_or_default()) }
        }
    }*/
}
