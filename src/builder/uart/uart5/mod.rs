use embassy_stm32::mode::Async;
use embassy_stm32::peripherals::{PC12, PD2, UART5};
use embassy_stm32::usart::{Config, ConfigError, Uart};
use crate::builder::uart::base::UartBase;

pub mod rx;
pub mod tx;

/// uart5 builder
pub struct Uart5Builder {
    /// uart5 base device
    pub base: UartBase<UART5>,
    /// tx pin
    pub tx: PC12,
    /// rx pin
    pub rx: PD2,
}

/// custom method
impl Uart5Builder {
    /// create builder
    #[deprecated(note = "no any dma support TxDma<UART5> or RxDma<UART5>")]
    #[inline]
    pub fn new(uart: UART5, tx: PC12, rx: PD2) -> Self {
        Self { base: UartBase::new(uart), tx, rx }
    }

    /// set uart config
    #[inline]
    pub fn config(mut self, config: Config) -> Self {
        self.base.set_config(config);
        self
    }

    /// build a serial port that supports read and write data
    #[deprecated(note = "no any dma support TxDma<UART5> or RxDma<UART5>")]
    #[inline]
    pub fn build(self) -> Result<Uart<'static, Async>, ConfigError> {
        Err(ConfigError::RxOrTxNotEnabled)
    }
}
