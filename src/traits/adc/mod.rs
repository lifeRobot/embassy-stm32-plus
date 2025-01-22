use embassy_stm32::adc::{Adc, Instance};

/// adc trait
pub trait AdcTrait: Instance {
    /// create adc, more see [Adc::new]
    fn build(self) -> Adc<'static, Self> {
        Adc::new(self)
    }
}

/// any adc support adc trait
impl<T: Instance> AdcTrait for T {}
