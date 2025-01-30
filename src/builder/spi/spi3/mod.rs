use embassy_stm32::mode::{Async, Blocking};
use embassy_stm32::peripherals::{DMA2_CH1, DMA2_CH2, PB3, PB4, PB5, SPI3};
use embassy_stm32::spi::{Config, Spi};
use crate::builder::spi::base::SpiBase;

pub mod rx;
pub mod tx;

/// spi3 builder
pub struct Spi3Builder {
    /// spi device
    pub base: SpiBase<SPI3>,
    /// sck pin
    pub sck: PB3,
    /// mosi pin
    pub mosi: PB5,
    /// miso pin
    pub miso: PB4,
}

/// custom method
impl Spi3Builder {
    /// create builder
    #[inline]
    pub fn new(spi: SPI3, sck: PB3, mosi: PB5, miso: PB4) -> Self {
        Self { base: SpiBase::new(spi), sck, mosi, miso }
    }

    /// set spi config
    #[inline]
    pub fn config(mut self, config: Config) -> Self {
        self.base.set_config(config);
        self
    }

    /// Create a new SPI driver.<br />
    /// more see [Spi::<Async>::new]
    #[inline]
    pub fn build(self, tx_dma: DMA2_CH2, rx_dma: DMA2_CH1) -> Spi<'static, Async> {
        Spi::new(self.base.spi, self.sck, self.mosi, self.miso, tx_dma, rx_dma, self.base.config.unwrap_or_default())
    }

    /// Create a new blocking SPI driver.<br />
    /// more see [Spi::<Blocking>::new_blocking]
    #[inline]
    pub fn build_blocking(self) -> Spi<'static, Blocking> {
        Spi::new_blocking(self.base.spi, self.sck, self.mosi, self.miso, self.base.config.unwrap_or_default())
    }
}
