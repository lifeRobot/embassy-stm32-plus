use embassy_stm32::mode::Async;
use embassy_stm32::peripherals::{PD2, UART5};
use embassy_stm32::usart::{Config, ConfigError, UartRx};
use crate::builder::uart::base::UartBase;

/// uart5 rx builder
pub struct Uart5RxBuilder {
    /// uart5 base device
    pub base: UartBase<UART5>,
    /// rx pin
    pub rx: PD2,
}

/// custom method
impl Uart5RxBuilder {
    /// create builder
    #[deprecated(note = "no any dma support RxDma<UART5>")]
    #[inline]
    pub fn new(uart: UART5, rx: PD2) -> Uart5RxBuilder {
        Self { base: UartBase::new(uart), rx }
    }

    /// set uart config
    #[inline]
    pub fn config(mut self, config: Config) -> Self {
        self.base.set_config(config);
        self
    }

    /// build uart rx that supports read data
    #[deprecated(note = "no any dma support RxDma<UART5>")]
    #[inline]
    pub fn build(self) -> Result<UartRx<'static, Async>, ConfigError> {
        Err(ConfigError::RxOrTxNotEnabled)
    }
}
