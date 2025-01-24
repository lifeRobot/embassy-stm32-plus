use embassy_stm32::dac::DacCh1;
use embassy_stm32::dma::NoDma;
use embassy_stm32::Peripheral;
use embassy_stm32::peripherals::{DAC1, PA4};

/// dac ch1 builder
pub struct DacCh1Builder {
    /// dac device
    pub dac: DAC1,
    /// dac ch1 pin
    pub ch1_pin: PA4,
}

/// custom method
impl DacCh1Builder {
    /// create builder
    #[inline]
    pub fn new(dac: DAC1, ch1_pin: PA4) -> Self {
        Self { dac, ch1_pin }
    }

    /// Create a new DacChannel instance, more see [DacCh1::new]
    #[inline]
    pub fn build<DMA>(self, dma: impl Peripheral<P=DMA> + 'static) -> DacCh1<'static, DAC1, DMA> {
        DacCh1::new(self.dac, dma, self.ch1_pin)
    }

    /// using NoDma create a new DacChannel instance, more see [DacCh1::new]
    #[inline]
    pub fn build_no_dma(self) -> DacCh1<'static, DAC1> {
        self.build(NoDma)
    }
}
