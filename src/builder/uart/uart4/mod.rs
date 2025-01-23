use embassy_stm32::{bind_interrupts, usart};
use embassy_stm32::mode::Async;
use embassy_stm32::peripherals::{DMA2_CH3, DMA2_CH5, PC10, PC11, UART4};
use embassy_stm32::usart::{Config, ConfigError, Uart};
use crate::builder::uart::base::UartBase;

pub mod rx;
pub mod tx;

bind_interrupts!(struct Irqs {
    UART4 => usart::InterruptHandler<UART4>;
});

/// uart4 builder
pub struct Uart4Builder {
    /// uart4 base device
    pub base: UartBase<UART4>,
    /// tx pin
    pub tx: PC10,
    /// rx pin
    pub rx: PC11,
}

/// custom method
impl Uart4Builder {
    /// create builder
    #[inline]
    pub fn new(uart: UART4, tx: PC10, rx: PC11) -> Self {
        Self { base: UartBase::new(uart), tx, rx }
    }

    /// set uart config
    #[inline]
    pub fn config(mut self, config: Config) -> Self {
        self.base.set_config(config);
        self
    }

    /// build a serial port that supports read and write data
    #[inline]
    pub fn build(self, tx_dma: DMA2_CH5, rx_dma: DMA2_CH3) -> Result<Uart<'static, Async>, ConfigError> {
        Uart::new(self.base.uart, self.rx, self.tx, Irqs, tx_dma, rx_dma, self.base.config.unwrap_or_default())
    }
}
