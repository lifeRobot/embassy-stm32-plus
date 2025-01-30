use embassy_stm32::usart::Config;

/// uart base data
pub struct UartBase<T> {
    /// uart devices
    pub uart: T,
    /// uart config
    pub config: Option<Config>,
}

/// custom method
impl<T> UartBase<T> {
    /// create base data
    #[inline]
    pub fn new(uart: T) -> Self {
        Self { uart, config: None }
    }

    /// set uart config
    #[inline]
    pub fn config(mut self, config: Config) -> Self {
        self.config = Some(config);
        self
    }

    /// set uart config
    #[inline]
    pub fn set_config(&mut self, config: Config) {
        self.config = Some(config)
    }
}
