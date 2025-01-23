use embassy_stm32::mode::Async;
use embassy_stm32::peripherals::{DMA2_CH5, PC10, UART4};
use embassy_stm32::usart::{Config, ConfigError, UartTx};
use crate::builder::uart::base::UartBase;

/// uart4 tx builder
pub struct Uart4TxBuilder {
    /// uart4 base device
    pub base: UartBase<UART4>,
    /// tx pin
    pub tx: PC10,
}

/// custom method
impl Uart4TxBuilder {
    /// create builder
    #[inline]
    pub fn new(uart: UART4, tx: PC10) -> Self {
        Self { base: UartBase::new(uart), tx }
    }

    /// set uart config
    #[inline]
    pub fn config(mut self, config: Config) -> Self {
        self.base.set_config(config);
        self
    }

    /// build uart tx that supports write data
    #[inline]
    pub fn build(self, tx_dma: DMA2_CH5) -> Result<UartTx<'static, Async>, ConfigError> {
        UartTx::new(self.base.uart, self.tx, tx_dma, self.base.config.unwrap_or_default())
    }
}
