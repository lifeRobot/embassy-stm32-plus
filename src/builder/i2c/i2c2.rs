use embassy_stm32::{bind_interrupts, i2c};
use embassy_stm32::i2c::{Config, I2c};
use embassy_stm32::mode::{Async, Blocking};
use embassy_stm32::peripherals::{DMA1_CH4, DMA1_CH5, I2C2, PB10, PB11};
use embassy_stm32::time::Hertz;
use crate::builder::i2c::base::I2cBase;

bind_interrupts!(struct Irqs {
    I2C2_ER => i2c::ErrorInterruptHandler<I2C2>;
    I2C2_EV => i2c::EventInterruptHandler<I2C2>;
});


/// i2c2 builder
pub struct I2c2Builder {
    /// i2c base
    pub base: I2cBase<I2C2>,
    /// scl pin
    pub scl: PB10,
    /// sda pin
    pub sda: PB11,
}

/// custom method
impl I2c2Builder {
    /// create builder
    #[inline]
    pub fn new(i2c: I2C2, scl: PB10, sda: PB11) -> Self {
        Self { base: I2cBase::new(i2c), scl, sda }
    }

    /// set i2c freq hertz
    #[inline]
    pub fn freq(mut self, freq: Hertz) -> Self {
        self.base.set_freq(freq);
        self
    }

    /// set i2c config
    #[inline]
    pub fn config(mut self, config: Config) -> Self {
        self.base.set_config(config);
        self
    }

    /// Create a new I2C driver, more see [`I2c::<Async>::new`]
    pub fn build(self, tx_dma: DMA1_CH4, rx_dma: DMA1_CH5) -> I2c<'static, Async> {
        let Self { base, scl, sda } = self;
        I2c::new(base.i2c, scl, sda, Irqs, tx_dma, rx_dma, base.freq.unwrap_or_else(|| { Hertz(1) }), base.config.unwrap_or_default())
    }

    /// Create a new I2C driver, more see [`I2c::<Blocking>::new_blocking`]
    pub fn build_blocking(self) -> I2c<'static, Blocking> {
        let Self { base, scl, sda } = self;
        I2c::new_blocking(base.i2c, scl, sda, base.freq.unwrap_or_else(|| { Hertz(1) }), base.config.unwrap_or_default())
    }
}
