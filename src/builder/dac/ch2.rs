use embassy_stm32::dac::DacCh2;
use embassy_stm32::dma::NoDma;
use embassy_stm32::Peripheral;
use embassy_stm32::peripherals::{DAC1, PA5};

/// dac ch2 builder
pub struct DacCh2Builder {
    /// dac device
    pub dac: DAC1,
    /// dac ch2 pin
    pub ch2_pin: PA5,
}

/// custom method
impl DacCh2Builder {
    /// create builder
    #[inline]
    pub fn new(dac: DAC1, ch2_pin: PA5) -> Self {
        Self { dac, ch2_pin }
    }

    /// Create a new DacChannel instance, more see [DacCh2::new]
    #[inline]
    pub fn build<DMA>(self, dma: impl Peripheral<P=DMA> + 'static) -> DacCh2<'static, DAC1, DMA> {
        DacCh2::new(self.dac, dma, self.ch2_pin)
    }

    /// using NoDma create a new DacChannel instance, more see [DacCh2::new]
    #[inline]
    pub fn build_no_dma(self) -> DacCh2<'static, DAC1> {
        self.build(NoDma)
    }
}
