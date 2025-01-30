use embassy_stm32::mode::{Async, Blocking};
use embassy_stm32::peripherals::{DMA2_CH2, PB3, PB5, SPI3};
use embassy_stm32::spi::{Config, Spi};
use crate::builder::spi::base::SpiBase;

/// spi3 tx builder
pub struct Spi3TxBuilder {
    /// spi device
    pub base: SpiBase<SPI3>,
    /// mosi pin
    pub mosi: PB5,
}

/// custom method
impl Spi3TxBuilder {
    /// create builder
    #[inline]
    pub fn new(spi: SPI3, mosi: PB5) -> Self {
        Spi3TxBuilder { base: SpiBase::new(spi), mosi }
    }

    /// set spi config
    #[inline]
    pub fn config(mut self, config: Config) -> Self {
        self.base.set_config(config);
        self
    }

    /// Create a new SPI driver, in TX-only mode (only MOSI pin, no MISO).<br />
    /// more see [Spi::<Async>::new_txonly]
    #[inline]
    pub fn build(self, sck: PB3, tx_dma: DMA2_CH2) -> Spi<'static, Async> {
        Spi::new_txonly(self.base.spi, sck, self.mosi, tx_dma, self.base.config.unwrap_or_default())
    }

    /// Create a new SPI driver, in TX-only mode, without SCK pin.<br />
    /// more see [Spi::<Async>::new_txonly_nosck]
    #[inline]
    pub fn build_nosck(self, tx_dma: DMA2_CH2) -> Spi<'static, Async> {
        Spi::new_txonly_nosck(self.base.spi, self.mosi, tx_dma, self.base.config.unwrap_or_default())
    }

    /// Create a new blocking SPI driver, in TX-only mode (only MOSI pin, no MISO).<br />
    /// more see [Spi::<Blocking>::new_blocking_txonly]
    #[inline]
    pub fn build_blocking(self, sck: PB3) -> Spi<'static, Blocking> {
        Spi::new_blocking_txonly(self.base.spi, sck, self.mosi, self.base.config.unwrap_or_default())
    }

    /// Create a new SPI driver, in TX-only mode, without SCK pin.<br />
    /// more see [Spi::<Blocking>::new_blocking_txonly_nosck]
    #[inline]
    pub fn build_blocking_nosck(self) -> Spi<'static, Blocking> {
        Spi::new_blocking_txonly_nosck(self.base.spi, self.mosi, self.base.config.unwrap_or_default())
    }
}