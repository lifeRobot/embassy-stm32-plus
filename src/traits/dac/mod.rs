use embassy_stm32::dac::{Dac, DacCh1, DacCh2, Instance};
use embassy_stm32::dma::NoDma;
use embassy_stm32::Peripheral;
use embassy_stm32::peripherals::{DAC, PA4, PA5};

/// dac trait
pub trait DacTrait<CH1, CH2>: Peripheral + Instance {
    fn build_ch_with_dma<DmaCh1, DmaCh2>(self, ch1: CH1, ch2: CH2, dma_ch1: impl Peripheral<P=DmaCh1> + 'static, dma_ch2: impl Peripheral<P=DmaCh2> + 'static) -> Dac<'static, Self, DmaCh1, DmaCh2>;

    fn build_ch1_with_dma<DmaCh1>(self, ch1: CH1, dma_ch1: impl Peripheral<P=DmaCh1> + 'static) -> DacCh1<'static, Self, DmaCh1>;

    fn build_ch2_with_dma<DmaCh2>(self, ch2: CH2, dma_ch2: impl Peripheral<P=DmaCh2> + 'static) -> DacCh2<'static, Self, DmaCh2>;

    fn build_ch(self, ch1: CH1, ch2: CH2) -> Dac<'static, Self> {
        self.build_ch_with_dma(ch1, ch2, NoDma, NoDma)
    }

    fn build_ch1(self, ch1: CH1) -> DacCh1<'static, Self> {
        self.build_ch1_with_dma(ch1, NoDma)
    }

    fn build_ch2(self, ch2: CH2) -> DacCh2<'static, Self> {
        self.build_ch2_with_dma(ch2, NoDma)
    }
}

/// support DAC
impl DacTrait<PA4, PA5> for DAC {
    fn build_ch_with_dma<DmaCh1, DmaCh2>(self, ch1: PA4, ch2: PA5, dma_ch1: impl Peripheral<P=DmaCh1> + 'static, dma_ch2: impl Peripheral<P=DmaCh2> + 'static) -> Dac<'static, Self, DmaCh1, DmaCh2> {
        Dac::new(self, dma_ch1, dma_ch2, ch1, ch2)
    }

    fn build_ch1_with_dma<DmaCh1>(self, ch1: PA4, dma_ch1: impl Peripheral<P=DmaCh1> + 'static) -> DacCh1<'static, Self, DmaCh1> {
        DacCh1::new(self, dma_ch1, ch1)
    }

    fn build_ch2_with_dma<DmaCh2>(self, ch2: PA5, dma_ch2: impl Peripheral<P=DmaCh2> + 'static) -> DacCh2<'static, Self, DmaCh2> {
        DacCh2::new(self, dma_ch2, ch2)
    }
}

