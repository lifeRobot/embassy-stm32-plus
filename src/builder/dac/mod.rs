use embassy_stm32::dac::Dac;
use embassy_stm32::dma::NoDma;
use embassy_stm32::Peripheral;
use embassy_stm32::peripherals::{DAC1, PA4, PA5};

pub mod ch1;
pub mod ch2;

/// dac builder
pub struct DacBuilder {
    /// dac device
    pub dac: DAC1,
    /// dac ch1 pin
    pub ch1_pin: PA4,
    /// dac ch2 pin
    pub ch2_pin: PA5,
}

/// custom method
impl DacBuilder {
    /// create builder
    #[inline]
    pub fn new(dac: DAC1, ch1_pin: PA4, ch2_pin: PA5) -> Self {
        Self { dac, ch1_pin, ch2_pin }
    }

    /// Create a new Dac instance, more see [Dac::new]
    #[inline]
    pub fn build<DmaCh1, DmaCh2>(self, dma_ch1: impl Peripheral<P=DmaCh1> + 'static, dma_ch2: impl Peripheral<P=DmaCh2> + 'static)
                                 -> Dac<'static, DAC1, DmaCh1, DmaCh2> {
        Dac::new(self.dac, dma_ch1, dma_ch2, self.ch1_pin, self.ch2_pin)
    }

    /// dma_ch2 using NoDma Create a new Dac instance, more see [Dac::new]
    #[inline]
    pub fn build_dma_ch1<DmaCh1>(self, dma_ch1: impl Peripheral<P=DmaCh1> + 'static) -> Dac<'static, DAC1, DmaCh1> {
        Self::build(self, dma_ch1, NoDma)
    }

    /// dma_ch1 using NoDma Create a new Dac instance, more see [Dac::new]
    #[inline]
    pub fn build_dma_ch2<DmaCh2>(self, dma_ch2: impl Peripheral<P=DmaCh2> + 'static) -> Dac<'static, DAC1, NoDma, DmaCh2> {
        Self::build(self, NoDma, dma_ch2)
    }

    /// using NoDma Create a new Dac instance, more see [Dac::new]
    #[inline]
    pub fn build_no_dma(self) -> Dac<'static, DAC1> {
        Self::build(self, NoDma, NoDma)
    }
}
