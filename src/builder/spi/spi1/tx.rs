use embassy_stm32::mode::{Async, Blocking};
use embassy_stm32::Peripheral;
use embassy_stm32::peripherals::{DMA1_CH3, PA7, PB5, SPI1};
use embassy_stm32::spi::{Config, SckPin, Spi};
use crate::builder::spi::base::SpiBase;
use crate::builder::spi::spi1::Spi1Sck;

/// spi1 mosi pin
pub enum Spi1Mosi {
    PA7(PA7),
    PB5(PB5),
}

/// spi1 tx builder
pub struct Spi1TxBuilder {
    /// spi device
    pub base: SpiBase<SPI1>,
    /// mosi pin
    pub mosi: Spi1Mosi,
}

/// custom method
impl Spi1TxBuilder {
    /// create builder
    #[inline]
    pub fn new(spi: SPI1, mosi: Spi1Mosi) -> Self {
        Self { base: SpiBase::new(spi), mosi }
    }

    /// set spi config
    #[inline]
    pub fn config(mut self, config: Config) -> Self {
        self.base.set_config(config);
        self
    }

    /// Create a new SPI driver, in TX-only mode (only MOSI pin, no MISO).<br />
    /// more see [Spi::<Async>::new_txonly]
    pub fn build(self, sck: Spi1Sck, tx_dma: DMA1_CH3) -> Spi<'static, Async> {
        match sck {
            Spi1Sck::PA5(pa5) => { self.build_mosi(pa5, tx_dma) }
            Spi1Sck::PB3(pb3) => { self.build_mosi(pb3, tx_dma) }
        }
    }

    /// build by mosi
    fn build_mosi(
        self,
        sck: impl Peripheral<P=impl SckPin<SPI1>> + 'static,
        tx_dma: DMA1_CH3) -> Spi<'static, Async> {
        match self.mosi {
            Spi1Mosi::PA7(pa7) => {
                Spi::new_txonly(self.base.spi, sck, pa7, tx_dma, self.base.config.unwrap_or_default())
            }
            Spi1Mosi::PB5(pb5) => {
                Spi::new_txonly(self.base.spi, sck, pb5, tx_dma, self.base.config.unwrap_or_default())
            }
        }
    }

    /// Create a new SPI driver, in TX-only mode, without SCK pin.<br />
    /// more see [Spi::<Async>::new_txonly_nosck]
    pub fn build_nosck(self, tx_dma: DMA1_CH3) -> Spi<'static, Async> {
        match self.mosi {
            Spi1Mosi::PA7(pa7) => {
                Spi::new_txonly_nosck(self.base.spi, pa7, tx_dma, self.base.config.unwrap_or_default())
            }
            Spi1Mosi::PB5(pb5) => {
                Spi::new_txonly_nosck(self.base.spi, pb5, tx_dma, self.base.config.unwrap_or_default())
            }
        }
    }

    /// Create a new blocking SPI driver, in TX-only mode (only MOSI pin, no MISO).<br />
    /// more see [Spi::<Blocking>::new_blocking_txonly]
    pub fn build_blocking(self, sck: Spi1Sck) -> Spi<'static, Blocking> {
        match sck {
            Spi1Sck::PA5(pa5) => { self.build_blocking_mosi(pa5) }
            Spi1Sck::PB3(pb3) => { self.build_blocking_mosi(pb3) }
        }
    }

    /// build blocking by mosi
    fn build_blocking_mosi(self, sck: impl Peripheral<P=impl SckPin<SPI1>> + 'static) -> Spi<'static, Blocking> {
        match self.mosi {
            Spi1Mosi::PA7(pa7) => {
                Spi::new_blocking_txonly(self.base.spi, sck, pa7, self.base.config.unwrap_or_default())
            }
            Spi1Mosi::PB5(pb5) => {
                Spi::new_blocking_txonly(self.base.spi, sck, pb5, self.base.config.unwrap_or_default())
            }
        }
    }

    /// Create a new SPI driver, in TX-only mode, without SCK pin.<br />
    /// more see [Spi::<Blocking>::new_blocking_txonly_nosck]
    #[inline]
    pub fn build_blocking_nosck(self) -> Spi<'static, Blocking> {
        match self.mosi {
            Spi1Mosi::PA7(pa7) => {
                Spi::new_blocking_txonly_nosck(self.base.spi, pa7, self.base.config.unwrap_or_default())
            }
            Spi1Mosi::PB5(pb5) => {
                Spi::new_blocking_txonly_nosck(self.base.spi, pb5, self.base.config.unwrap_or_default())
            }
        }
    }
}
