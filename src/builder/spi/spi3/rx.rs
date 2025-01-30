use embassy_stm32::mode::{Async, Blocking};
use embassy_stm32::peripherals::{DMA2_CH1, DMA2_CH2, PB3, PB4, SPI3};
use embassy_stm32::spi::{Config, Spi};
use crate::builder::spi::base::SpiBase;

/// spi3 rx builder
pub struct Spi3RxBuilder {
    /// spi device
    pub base: SpiBase<SPI3>,
    /// sck pin
    pub sck: PB3,
    /// miso pin
    pub miso: PB4,
}

/// custom method
impl Spi3RxBuilder {
    /// create builder
    #[inline]
    pub fn new(spi: SPI3, sck: PB3, miso: PB4) -> Self {
        Self { base: SpiBase::new(spi), sck, miso }
    }

    /// set spi config
    #[inline]
    pub fn config(mut self, config: Config) -> Self {
        self.base.set_config(config);
        self
    }

    /// Create a new SPI driver, in RX-only mode (only MISO pin, no MOSI).<br />
    /// more see [Spi::<Async>::new_rxonly]
    #[inline]
    pub fn build(self, tx_dma: DMA2_CH2, rx_dma: DMA2_CH1) -> Spi<'static, Async> {
        Spi::new_rxonly(self.base.spi, self.sck, self.miso, tx_dma, rx_dma, self.base.config.unwrap_or_default())
    }

    /// Create a new blocking SPI driver, in RX-only mode (only MISO pin, no MOSI).<br />
    /// more see [Spi::<Blocking>::new_blocking_rxonly]
    #[inline]
    pub fn build_blocking(self) -> Spi<'static, Blocking> {
        Spi::new_blocking_rxonly(self.base.spi, self.sck, self.miso, self.base.config.unwrap_or_default())
    }
}
