use embassy_stm32::peripherals::{DAC1, PA4, PA5};
use crate::builder::dac::ch1::DacCh1Builder;
use crate::builder::dac::ch2::DacCh2Builder;
use crate::builder::dac::DacBuilder;

/// dac trait
pub trait DacTrait {
    /// create dac builder
    fn builder(self, ch1_pin: PA4, ch2_pin: PA5) -> DacBuilder;

    /// create dac ch1 builder
    fn ch1_builder(self, ch1_pin: PA4) -> DacCh1Builder;

    /// create dac ch2 builder
    fn ch2_builder(self, ch2_pin: PA5) -> DacCh2Builder;
}

/// dac support dac trait
impl DacTrait for DAC1 {
    #[inline]
    fn builder(self, ch1_pin: PA4, ch2_pin: PA5) -> DacBuilder {
        DacBuilder::new(self, ch1_pin, ch2_pin)
    }

    #[inline]
    fn ch1_builder(self, ch1_pin: PA4) -> DacCh1Builder {
        DacCh1Builder::new(self, ch1_pin)
    }

    #[inline]
    fn ch2_builder(self, ch2_pin: PA5) -> DacCh2Builder {
        DacCh2Builder::new(self, ch2_pin)
    }
}
