use embassy_stm32::adc::{Adc, Instance};
use embassy_time::Delay;
use embedded_hal::blocking::delay::DelayUs;

/// adc trait
pub trait AdcTrait: Instance {
    fn build(self) -> Adc<'static, Self> {
        self.build_with_delay(&mut Delay)
    }

    fn build_with_delay(self, delay: &mut impl DelayUs<u32>) -> Adc<'static, Self> {
        Adc::new(self, delay)
    }
}
