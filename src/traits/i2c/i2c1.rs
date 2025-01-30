use embassy_stm32::peripherals::I2C1;
use crate::builder::i2c::i2c1::{I2c1Builder, I2c1Scl, I2c1Sda};

/// i2c1 trait
pub trait I2c1Trait {
    /// create i2c1 builder
    fn builder(self, scl: I2c1Scl, sda: I2c1Sda) -> I2c1Builder;
}

/// i2c1 support i2c1 trait
impl I2c1Trait for I2C1 {
    #[inline]
    fn builder(self, scl: I2c1Scl, sda: I2c1Sda) -> I2c1Builder {
        I2c1Builder::new(self, scl, sda)
    }
}
