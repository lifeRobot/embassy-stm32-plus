use embassy_stm32::mode::{Async, Blocking};
use embassy_stm32::peripherals::{DMA1_CH5, PB13, PB15, SPI2};
use embassy_stm32::spi::{Config, Spi};
use crate::builder::spi::base::SpiBase;

/// spi2 tx builder
pub struct Spi2TxBuilder {
    /// spi device
    pub base: SpiBase<SPI2>,
    /// mosi pin
    pub mosi: PB15,
}

/// custom method
impl Spi2TxBuilder {
    /// create builder
    #[inline]
    pub fn new(spi: SPI2, mosi: PB15) -> Self {
        Self { base: SpiBase::new(spi), mosi }
    }

    /// set spi config
    #[inline]
    pub fn config(mut self, config: Config) -> Self {
        self.base.set_config(config);
        self
    }

    /// Create a new SPI driver, in TX-only mode (only MOSI pin, no MISO).<br />
    /// more see [`Spi::<Async>::new_txonly`]
    #[inline]
    pub fn build(self, sck: PB13, tx_dma: DMA1_CH5) -> Spi<'static, Async> {
        Spi::new_txonly(self.base.spi, sck, self.mosi, tx_dma, self.base.config.unwrap_or_default())
    }

    /// Create a new SPI driver, in TX-only mode, without SCK pin.<br />
    /// more see [`Spi::<Async>::new_txonly_nosck`]
    #[inline]
    pub fn build_nosck(self, tx_dma: DMA1_CH5) -> Spi<'static, Async> {
        Spi::new_txonly_nosck(self.base.spi, self.mosi, tx_dma, self.base.config.unwrap_or_default())
    }

    /// Create a new blocking SPI driver, in TX-only mode (only MOSI pin, no MISO).<br />
    /// more see [`Spi::<Blocking>::new_blocking_txonly`]
    #[inline]
    pub fn build_blocking(self, sck: PB13) -> Spi<'static, Blocking> {
        Spi::new_blocking_txonly(self.base.spi, sck, self.mosi, self.base.config.unwrap_or_default())
    }

    /// Create a new SPI driver, in TX-only mode, without SCK pin.<br />
    /// more see [`Spi::<Blocking>::new_blocking_txonly_nosck`]
    #[inline]
    pub fn build_blocking_nosck(self) -> Spi<'static, Blocking> {
        Spi::new_blocking_txonly_nosck(self.base.spi, self.mosi, self.base.config.unwrap_or_default())
    }
}
