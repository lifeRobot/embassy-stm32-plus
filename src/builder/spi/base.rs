use embassy_stm32::spi::Config;

/// spi base data
pub struct SpiBase<T> {
    /// spi device
    pub spi: T,
    /// spi config
    pub config: Option<Config>,
}

/// custom method
impl<T> SpiBase<T> {
    /// create base data
    #[inline]
    pub fn new(spi: T) -> Self {
        Self { spi, config: None }
    }

    /// set spi config
    #[inline]
    pub fn config(mut self, config: Config) -> Self {
        self.config = Some(config);
        self
    }

    /// set spi config
    #[inline]
    pub fn set_config(&mut self, config: Config) {
        self.config = Some(config);
    }
}
