use embassy_stm32::mode::{Async, Blocking};
use embassy_stm32::peripherals::{DMA1_CH4, DMA1_CH5, PB13, PB14, SPI2};
use embassy_stm32::spi::{Config, Spi};
use crate::builder::spi::base::SpiBase;

/// spi2 rx builder
pub struct Spi2RxBuilder {
    /// spi device
    pub base: SpiBase<SPI2>,
    /// sck pin
    pub sck: PB13,
    /// miso pin
    pub miso: PB14,
}

/// custom method
impl Spi2RxBuilder {
    /// create builder
    #[inline]
    pub fn new(spi: SPI2, sck: PB13, miso: PB14) -> Self {
        Self { base: SpiBase::new(spi), sck, miso }
    }

    /// set spi config
    #[inline]
    pub fn config(mut self, config: Config) -> Self {
        self.base.set_config(config);
        self
    }

    /// Create a new SPI driver, in RX-only mode (only MISO pin, no MOSI).<br />
    /// more see [`Spi::<Async>::new_rxonly`]
    #[inline]
    pub fn build(self, tx_dma: DMA1_CH5, rx_dma: DMA1_CH4) -> Spi<'static, Async> {
        Spi::new_rxonly(self.base.spi, self.sck, self.miso, tx_dma, rx_dma, self.base.config.unwrap_or_default())
    }

    /// Create a new blocking SPI driver, in RX-only mode (only MISO pin, no MOSI).<br />
    /// more see [`Spi::<Blocking>::new_blocking_rxonly`]
    #[inline]
    pub fn build_blocking(self) -> Spi<'static, Blocking> {
        Spi::new_blocking_rxonly(self.base.spi, self.sck, self.miso, self.base.config.unwrap_or_default())
    }
}
