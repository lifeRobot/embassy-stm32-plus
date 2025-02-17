use embassy_stm32::mode::{Async, Blocking};
use embassy_stm32::Peripheral;
use embassy_stm32::peripherals::{DMA2_CH1, DMA2_CH2, PB4, SPI3};
#[cfg(SPI3_PC11)]
use embassy_stm32::peripherals::PC11;
use embassy_stm32::spi::{Config, SckPin, Spi};
use crate::builder::spi::base::SpiBase;
use crate::builder::spi::spi3::Spi3Sck;

/// spi3 miso pin
pub enum Spi3Miso {
    PB4(PB4),
    #[cfg(SPI3_PC11)]
    PC11(PC11),
}

/// spi3 rx builder
pub struct Spi3RxBuilder {
    /// spi device
    pub base: SpiBase<SPI3>,
    /// sck pin
    pub sck: Spi3Sck,
    /// miso pin
    pub miso: Spi3Miso,
}

/// custom method
impl Spi3RxBuilder {
    /// create builder
    #[inline]
    pub fn new(spi: SPI3, sck: Spi3Sck, miso: Spi3Miso) -> Self {
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
    pub fn build(self, tx_dma: DMA2_CH2, rx_dma: DMA2_CH1) -> Spi<'static, Async> {
        let Self { base, sck, miso } = self;
        match sck {
            Spi3Sck::PB3(pb3) => { Self::build_miso(base, pb3, miso, tx_dma, rx_dma) }
            #[cfg(SPI3_PC10)]
            Spi3Sck::PC10(pc10) => { Self::build_miso(base, pc10, miso, tx_dma, rx_dma) }
        }
    }

    /// build by miso
    fn build_miso(
        base: SpiBase<SPI3>,
        sck: impl Peripheral<P=impl SckPin<SPI3>> + 'static,
        miso: Spi3Miso,
        tx_dma: DMA2_CH2,
        rx_dma: DMA2_CH1) -> Spi<'static, Async> {
        match miso {
            Spi3Miso::PB4(pb4) => { Spi::new_rxonly(base.spi, sck, pb4, tx_dma, rx_dma, base.config.unwrap_or_default()) }
            #[cfg(SPI3_PC11)]
            Spi3Miso::PC11(pc11) => { Spi::new_rxonly(base.spi, sck, pc11, tx_dma, rx_dma, base.config.unwrap_or_default()) }
        }
    }

    /// Create a new blocking SPI driver, in RX-only mode (only MISO pin, no MOSI).<br />
    /// more see [`Spi::<Blocking>::new_blocking_rxonly`]
    #[inline]
    pub fn build_blocking(self) -> Spi<'static, Blocking> {
        let Self { base, sck, miso } = self;
        match sck {
            Spi3Sck::PB3(pb3) => { Self::build_blocking_miso(base, pb3, miso) }
            #[cfg(SPI3_PC10)]
            Spi3Sck::PC10(pc10) => { Self::build_blocking_miso(base, pc10, miso) }
        }
    }

    /// build blocking by miso
    fn build_blocking_miso(
        base: SpiBase<SPI3>,
        sck: impl Peripheral<P=impl SckPin<SPI3>> + 'static,
        miso: Spi3Miso) -> Spi<'static, Blocking> {
        match miso {
            Spi3Miso::PB4(pb4) => { Spi::new_blocking_rxonly(base.spi, sck, pb4, base.config.unwrap_or_default()) }
            #[cfg(SPI3_PC11)]
            Spi3Miso::PC11(pc11) => { Spi::new_blocking_rxonly(base.spi, sck, pc11, base.config.unwrap_or_default()) }
        }
    }
}
