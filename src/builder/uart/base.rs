use embassy_stm32::Peripheral;
use embassy_stm32::usart::Config;

/// uart base data
pub struct UartBase<T: Peripheral> {
    /// uart config
    pub config: Option<Config>,
    /// uart1 devices
    pub uart: T,
}

/// custom method
impl<T: Peripheral> UartBase<T> {
    /// create base data
    #[inline]
    pub fn new(uart: T) -> Self {
        Self { uart, config: None }
    }

    /// set uart config
    #[inline]
    pub fn set_config(&mut self, config: Config) {
        self.config = Some(config)
    }
}
