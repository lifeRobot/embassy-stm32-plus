use embassy_stm32::i2c::Config;
use embassy_stm32::time::Hertz;

/// i2c base data
pub struct I2cBase<T> {
    /// i2c device
    pub i2c: T,
    /// hertz
    pub freq: Option<Hertz>,
    /// i2c config
    pub config: Option<Config>,
}

/// custom method
impl<T> I2cBase<T> {
    /// create base data
    #[inline]
    pub fn new(i2c: T) -> Self {
        Self { i2c, freq: None, config: None }
    }

    /// set i2c freq hertz
    #[inline]
    pub fn freq(mut self, freq: Hertz) -> Self {
        self.freq = Some(freq);
        self
    }

    /// set i2c config
    #[inline]
    pub fn config(mut self, config: Config) -> Self {
        self.config = Some(config);
        self
    }

    /// set i2c freq hertz
    #[inline]
    pub fn set_freq(&mut self, freq: Hertz) {
        self.freq = Some(freq);
    }

    /// set i2c config
    #[inline]
    pub fn set_config(&mut self, config: Config) {
        self.config = Some(config);
    }
}
