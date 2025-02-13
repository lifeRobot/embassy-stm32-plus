use crate::builder::uart::base::UartBase;
use embassy_stm32::mode::Async;
#[cfg(not(STM32C0))]
use embassy_stm32::peripherals::DMA1_CH7;
use embassy_stm32::peripherals::{PA0, PA2, USART2};
#[cfg(USART2_PA14)]
use embassy_stm32::peripherals::PA14;
#[cfg(USART2_TX_PA4)]
use embassy_stm32::peripherals::PA4;
#[cfg(USART2_PA8)]
use embassy_stm32::peripherals::PA8;
#[cfg(USART2_PB7)]
use embassy_stm32::peripherals::PB7;
#[cfg(USART2_PB8)]
use embassy_stm32::peripherals::PB8;
#[cfg(PD3)]
use embassy_stm32::peripherals::PD3;
#[cfg(PD5)]
use embassy_stm32::peripherals::PD5;
use embassy_stm32::usart::{Config, ConfigError, TxPin, UartTx};
#[cfg(STM32C0)]
use embassy_stm32::usart::TxDma;
use embassy_stm32::Peripheral;

/// uart2 tx pin
pub enum Uart2Tx {
    PA2(PA2),
    #[cfg(USART2_TX_PA4)]
    PA4(PA4),
    #[cfg(USART2_PA8)]
    PA8(PA8),
    #[cfg(USART2_PA14)]
    PA14(PA14),
    #[cfg(PD5)]
    PD5(PD5),
}

/// uart2 cts pin
pub enum Uart2Cts {
    PA0(PA0),
    #[cfg(USART2_PB7)]
    PB7(PB7),
    #[cfg(USART2_PB8)]
    PB8(PB8),
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
                #[cfg(USART2_TX_PA4)]
                Uart2Tx::PA4(pa4) => Self::build_cts(pa4, tx_dma, self.base, self.cts),
                #[cfg(USART2_PA8)]
                Uart2Tx::PA8(pa8) => Self::build_cts(pa8, tx_dma, self.base, self.cts),
                #[cfg(USART2_PA14)]
                Uart2Tx::PA14(pa14) => Self::build_cts(pa14, tx_dma, self.base, self.cts),
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
                #[cfg(USART2_PB7)]
                Uart2Cts::PB7(pb7) => { UartTx::new_with_cts(base.uart, tx, pb7, tx_dma, base.config.unwrap_or_default()) }
                #[cfg(USART2_PB8)]
                Uart2Cts::PB8(pb8) => { UartTx::new_with_cts(base.uart, tx, pb8, tx_dma, base.config.unwrap_or_default()) }
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
    uart2_tx_build!(impl Peripheral<P = impl TxDma<USART2>> + 'static);
    #[cfg(not(STM32C0))]
    uart2_tx_build!(DMA1_CH7);
}
