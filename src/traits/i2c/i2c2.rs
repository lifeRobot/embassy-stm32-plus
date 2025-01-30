use embassy_stm32::peripherals::{I2C2, PB10, PB11};
use crate::builder::i2c::i2c2::I2c2Builder;

/// i2c2 trait
pub trait I2c2Trait {
    /// create i2c2 builder
    fn builder(self, scl: PB10, sda: PB11) -> I2c2Builder;
}

/// i2c2 support i2c2 trait
impl I2c2Trait for I2C2 {
    #[inline]
    fn builder(self, scl: PB10, sda: PB11) -> I2c2Builder {
        I2c2Builder::new(self, scl, sda)
    }
}
