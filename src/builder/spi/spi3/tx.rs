use embassy_stm32::mode::{Async, Blocking};
use embassy_stm32::Peripheral;
use embassy_stm32::peripherals::{DMA2_CH2, PB5, SPI3};
#[cfg(SPI3_PC12)]
use embassy_stm32::peripherals::PC12;
use embassy_stm32::spi::{Config, SckPin, Spi};
use crate::builder::spi::base::SpiBase;
use crate::builder::spi::spi3::Spi3Sck;

/// spi3 mosi pin
pub enum Spi3Mosi {
    PB5(PB5),
    #[cfg(SPI3_PC12)]
    PC12(PC12),
}

/// spi3 tx builder
pub struct Spi3TxBuilder {
    /// spi device
    pub base: SpiBase<SPI3>,
    /// mosi pin
    pub mosi: Spi3Mosi,
}

/// custom method
impl Spi3TxBuilder {
    /// create builder
    #[inline]
    pub fn new(spi: SPI3, mosi: Spi3Mosi) -> Self {
        Spi3TxBuilder { base: SpiBase::new(spi), mosi }
    }

    /// set spi config
    #[inline]
    pub fn config(mut self, config: Config) -> Self {
        self.base.set_config(config);
        self
    }

    /// Create a new SPI driver, in TX-only mode (only MOSI pin, no MISO).<br />
    /// more see [`Spi::<Async>::new_txonly`]
    pub fn build(self, sck: Spi3Sck, tx_dma: DMA2_CH2) -> Spi<'static, Async> {
        match sck {
            Spi3Sck::PB3(pb3) => { self.build_mosi(pb3, tx_dma) }
            #[cfg(SPI3_PC10)]
            Spi3Sck::PC10(pc10) => { self.build_mosi(pc10, tx_dma) }
        }
    }

    /// build by mosi
    fn build_mosi(self, sck: impl Peripheral<P=impl SckPin<SPI3>> + 'static, tx_dma: DMA2_CH2) -> Spi<'static, Async> {
        match self.mosi {
            Spi3Mosi::PB5(pb5) => { Spi::new_txonly(self.base.spi, sck, pb5, tx_dma, self.base.config.unwrap_or_default()) }
            #[cfg(SPI3_PC12)]
            Spi3Mosi::PC12(pc12) => { Spi::new_txonly(self.base.spi, sck, pc12, tx_dma, self.base.config.unwrap_or_default()) }
        }
    }

    /// Create a new SPI driver, in TX-only mode, without SCK pin.<br />
    /// more see [`Spi::<Async>::new_txonly_nosck`]
    pub fn build_nosck(self, tx_dma: DMA2_CH2) -> Spi<'static, Async> {
        match self.mosi {
            Spi3Mosi::PB5(pb5) => { Spi::new_txonly_nosck(self.base.spi, pb5, tx_dma, self.base.config.unwrap_or_default()) }
            #[cfg(SPI3_PC12)]
            Spi3Mosi::PC12(pc12) => { Spi::new_txonly_nosck(self.base.spi, pc12, tx_dma, self.base.config.unwrap_or_default()) }
        }
    }

    /// Create a new blocking SPI driver, in TX-only mode (only MOSI pin, no MISO).<br />
    /// more see [`Spi::<Blocking>::new_blocking_txonly`]
    pub fn build_blocking(self, sck: Spi3Sck) -> Spi<'static, Blocking> {
        match sck {
            Spi3Sck::PB3(pb3) => { self.build_blocking_mosi(pb3) }
            #[cfg(SPI3_PC10)]
            Spi3Sck::PC10(pc10) => { self.build_blocking_mosi(pc10) }
        }
    }

    /// build blocking by mosi
    fn build_blocking_mosi(self, sck: impl Peripheral<P=impl SckPin<SPI3>> + 'static) -> Spi<'static, Blocking> {
        match self.mosi {
            Spi3Mosi::PB5(pb5) => { Spi::new_blocking_txonly(self.base.spi, sck, pb5, self.base.config.unwrap_or_default()) }
            #[cfg(SPI3_PC12)]
            Spi3Mosi::PC12(pc12) => { Spi::new_blocking_txonly(self.base.spi, sck, pc12, self.base.config.unwrap_or_default()) }
        }
    }

    /// Create a new SPI driver, in TX-only mode, without SCK pin.<br />
    /// more see [`Spi::<Blocking>::new_blocking_txonly_nosck`]
    #[inline]
    pub fn build_blocking_nosck(self) -> Spi<'static, Blocking> {
        match self.mosi {
            Spi3Mosi::PB5(pb5) => { Spi::new_blocking_txonly_nosck(self.base.spi, pb5, self.base.config.unwrap_or_default()) }
            #[cfg(SPI3_PC12)]
            Spi3Mosi::PC12(pc12) => { Spi::new_blocking_txonly_nosck(self.base.spi, pc12, self.base.config.unwrap_or_default()) }
        }
    }
}