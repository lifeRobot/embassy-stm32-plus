use embassy_stm32::mode::Async;
use embassy_stm32::peripherals::{DMA2_CH3, PC11, UART4};
use embassy_stm32::usart::{Config, ConfigError, UartRx};
use crate::builder::uart::base::UartBase;
use crate::builder::uart::uart4::Irqs;

/// uart4 rx builder
pub struct Uart4RxBuilder {
    /// uart4 base device
    pub base: UartBase<UART4>,
    /// rx pin
    pub rx: PC11,
}

/// custom method
impl Uart4RxBuilder {
    /// create builder
    #[inline]
    pub fn new(uart: UART4, rx: PC11) -> Self {
        Self { base: UartBase::new(uart), rx }
    }

    /// set uart config
    #[inline]
    pub fn config(mut self, config: Config) -> Self {
        self.base.set_config(config);
        self
    }

    /// build uart rx that supports read data
    #[inline]
    pub fn build(self, rx_dma: DMA2_CH3) -> Result<UartRx<'static, Async>, ConfigError> {
        UartRx::new(self.base.uart, Irqs, self.rx, rx_dma, self.base.config.unwrap_or_default())
    }
}
