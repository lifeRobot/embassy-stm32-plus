use embassy_stm32::mode::Async;
use embassy_stm32::Peripheral;
#[cfg(not(STM32C0))]
use embassy_stm32::peripherals::DMA1_CH4;
use embassy_stm32::peripherals::{PA11, USART1};
#[cfg(USART1_PA0)]
use embassy_stm32::peripherals::PA0;
#[cfg(PA9)]
use embassy_stm32::peripherals::PA9;
#[cfg(USART1_PB4)]
use embassy_stm32::peripherals::PB4;
#[cfg(USART1_PB6)]
use embassy_stm32::peripherals::PB6;
#[cfg(USART1_PC14)]
use embassy_stm32::peripherals::PC14;
use embassy_stm32::usart::{Config, ConfigError, TxPin, UartTx};
#[cfg(STM32C0)]
use embassy_stm32::usart::TxDma;
use crate::builder::uart::base::UartBase;

/// uart1 tx pin
pub enum Uart1Tx {
    #[cfg(USART1_PA0)]
    PA0(PA0),
    #[cfg(PA9)]
    PA9(PA9),
    #[cfg(USART1_PB6)]
    PB6(PB6),
    #[cfg(USART1_PC14)]
    PC14(PC14),
}

/// uart1 cts pin
pub enum Uart1Cts {
    #[cfg(USART1_PA11)]
    PA11(PA11),
    #[cfg(USART1_PB4)]
    PB4(PB4),
    #[cfg(USART1_CTS_PB6)]
    PB6(PB6),
}

/// uart1 tx builder
pub struct Uart1TxBuilder {
    /// uart1 base data
    pub base: UartBase<USART1>,
    /// tx pin
    pub tx: Uart1Tx,
    /// use cts
    pub cts: Option<Uart1Cts>,
}

/// uart1 tx build
macro_rules! uart1_tx_build {
    ($tx_dma:ty) => {
        /// build uart tx that supports write data
        pub fn build(self, tx_dma: $tx_dma) -> Result<UartTx<'static, Async>, ConfigError> {
            match self.tx {
                #[cfg(USART1_PA0)]
                Uart1Tx::PA0(pa0) => { Self::build_cts(pa0, tx_dma, self.base, self.cts) }
                #[cfg(PA9)]
                Uart1Tx::PA9(pa9) => { Self::build_cts(pa9, tx_dma, self.base, self.cts) }
                #[cfg(USART1_PB6)]
                Uart1Tx::PB6(pb6) => { Self::build_cts(pb6, tx_dma, self.base, self.cts) }
                #[cfg(USART1_PC14)]
                Uart1Tx::PC14(pc14) => { Self::build_cts(pc14, tx_dma, self.base, self.cts) }
            }
        }

        /// build cts or default
        fn build_cts(
            tx: impl Peripheral<P=impl TxPin<USART1>> + 'static,
            tx_dma: $tx_dma,
            base: UartBase<USART1>,
            cts: Option<Uart1Cts>)
            -> Result<UartTx<'static, Async>, ConfigError> {
            let cts = crate::match_some_return!(cts,
                UartTx::new(base.uart, tx, tx_dma, base.config.unwrap_or_default()));
            match cts {
                #[cfg(USART1_PA11)]
                Uart1Cts::PA11(pa11) => { UartTx::new_with_cts(base.uart, tx, pa11, tx_dma, base.config.unwrap_or_default()) }
                #[cfg(USART1_PB4)]
                Uart1Cts::PB4(pb4) => { UartTx::new_with_cts(base.uart, tx, pb4, tx_dma, base.config.unwrap_or_default()) }
                #[cfg(USART1_CTS_PB6)]
                Uart1Cts::PB6(pb6) => { UartTx::new_with_cts(base.uart, tx, pb6, tx_dma, base.config.unwrap_or_default()) }
            }
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
    pub fn cts(mut self, cts: Uart1Cts) -> Self {
        self.cts = Some(cts);
        self
    }

    #[cfg(STM32C0)]
    uart1_tx_build!(impl Peripheral<P = impl TxDma<USART1>> + 'static);
    #[cfg(not(STM32C0))]
    uart1_tx_build!(DMA1_CH4);
}
