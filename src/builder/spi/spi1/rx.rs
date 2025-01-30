use embassy_stm32::mode::{Async, Blocking};
use embassy_stm32::Peripheral;
use embassy_stm32::peripherals::{DMA1_CH2, DMA1_CH3, PA6, PB4, SPI1};
use embassy_stm32::spi::{Config, SckPin, Spi};
use crate::builder::spi::base::SpiBase;
use crate::builder::spi::spi1::Spi1Sck;

/// spi1 miso pin
pub enum Spi1Miso {
    PA6(PA6),
    PB4(PB4),
}

/// spi1 rx builder
pub struct Spi1RxBuilder {
    /// spi device
    pub base: SpiBase<SPI1>,
    /// sck pin
    pub sck: Spi1Sck,
    /// miso pin
    pub miso: Spi1Miso,
}

/// custom method
impl Spi1RxBuilder {
    /// create builder
    #[inline]
    pub fn new(spi: SPI1, sck: Spi1Sck, miso: Spi1Miso) -> Self {
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
    pub fn build(self, tx_dma: DMA1_CH3, rx_dma: DMA1_CH2) -> Spi<'static, Async> {
        let Self { base, sck, miso } = self;
        match sck {
            Spi1Sck::PA5(pa5) => { Self::build_miso(base, pa5, miso, tx_dma, rx_dma) }
            Spi1Sck::PB3(pb3) => { Self::build_miso(base, pb3, miso, tx_dma, rx_dma) }
        }
    }

    /// build by miso
    fn build_miso(
        base: SpiBase<SPI1>,
        sck: impl Peripheral<P=impl SckPin<SPI1>> + 'static,
        miso: Spi1Miso,
        tx_dma: DMA1_CH3,
        rx_dma: DMA1_CH2) -> Spi<'static, Async> {
        match miso {
            Spi1Miso::PA6(pa6) => {
                Spi::new_rxonly(base.spi, sck, pa6, tx_dma, rx_dma, base.config.unwrap_or_default())
            }
            Spi1Miso::PB4(pb4) => {
                Spi::new_rxonly(base.spi, sck, pb4, tx_dma, rx_dma, base.config.unwrap_or_default())
            }
        }
    }


    /// Create a new blocking SPI driver, in RX-only mode (only MISO pin, no MOSI).<br />
    /// more see [Spi::<Blocking>::new_blocking_rxonly]
    pub fn build_blocking(self) -> Spi<'static, Blocking> {
        let Self { base, sck, miso } = self;
        match sck {
            Spi1Sck::PA5(pa5) => { Self::build_blocking_miso(base, pa5, miso) }
            Spi1Sck::PB3(pb3) => { Self::build_blocking_miso(base, pb3, miso) }
        }
    }

    /// build blocking by miso
    fn build_blocking_miso(
        base: SpiBase<SPI1>,
        sck: impl Peripheral<P=impl SckPin<SPI1>> + 'static,
        miso: Spi1Miso) -> Spi<'static, Blocking> {
        match miso {
            Spi1Miso::PA6(pa6) => { Spi::new_blocking_rxonly(base.spi, sck, pa6, base.config.unwrap_or_default()) }
            Spi1Miso::PB4(pb4) => { Spi::new_blocking_rxonly(base.spi, sck, pb4, base.config.unwrap_or_default()) }
        }
    }
}