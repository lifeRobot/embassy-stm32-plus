use embassy_stm32::{bind_interrupts, usart};
use embassy_stm32::peripherals::USART1;
use embassy_stm32::usart::Config;

bind_interrupts!(pub(crate) struct Irqs {
    USART1 => usart::InterruptHandler<USART1>;
});


/// uart1 base data
pub struct Uart1Base {
    /// uart config
    pub config: Option<Config>,
    /// uart1 devices
    pub uart: USART1,
}

/// custom method
impl Uart1Base {
    /// create base data
    #[inline]
    pub fn new(uart: USART1) -> Self {
        Self { uart, config: None }
    }

    /// set uart config
    #[inline]
    pub fn set_config(&mut self, config: Config) {
        self.config = Some(config)
    }
}
