use embassy_stm32::mode::Async;
use embassy_stm32::peripherals::{PC12, UART5};
use embassy_stm32::usart::{Config, ConfigError, UartTx};
use crate::builder::uart::base::UartBase;

/// uart5 tx builder
pub struct Uart5TxBuilder {
    /// uart5 base device
    pub base: UartBase<UART5>,
    /// tx pin
    pub tx: PC12,
}

/// custom method
impl Uart5TxBuilder {
    /// create builder
    #[deprecated(note = "no any dma support TxDma<UART5>")]
    #[inline]
    pub fn new(uart: UART5, tx: PC12) -> Uart5TxBuilder {
        Self { base: UartBase::new(uart), tx }
    }

    /// set uart config
    #[inline]
    pub fn config(mut self, config: Config) -> Self {
        self.base.set_config(config);
        self
    }

    /// build uart tx that supports write data
    #[deprecated(note = "no any dma support TxDma<UART5>")]
    #[inline]
    pub fn build(self) -> Result<UartTx<'static, Async>, ConfigError> {
        Err(ConfigError::RxOrTxNotEnabled)
    }
}
