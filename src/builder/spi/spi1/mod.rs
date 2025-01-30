use embassy_stm32::mode::{Async, Blocking};
use embassy_stm32::Peripheral;
use embassy_stm32::peripherals::{DMA1_CH2, DMA1_CH3, PA5, PB3, SPI1};
use embassy_stm32::spi::{Config, MisoPin, MosiPin, Spi};
use crate::builder::spi::base::SpiBase;
use crate::builder::spi::spi1::rx::{Spi1Miso, Spi1RxBuilder};
use crate::builder::spi::spi1::tx::Spi1Mosi;

pub mod rx;
pub mod tx;

/// spi1 sck pin
pub enum Spi1Sck {
    PA5(PA5),
    PB3(PB3),
}

/// spi1 builder
pub struct Spi1Builder {
    /// spi device
    pub base: SpiBase<SPI1>,
    /// sck pin
    pub sck: Spi1Sck,
    /// mosi pin
    pub mosi: Spi1Mosi,
    /// miso pin
    pub miso: Spi1Miso,
}

/// custom method
impl Spi1Builder {
    /// create builder
    #[inline]
    pub fn new(spi: SPI1, sck: Spi1Sck, mosi: Spi1Mosi, miso: Spi1Miso) -> Self {
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
    pub fn build(self, tx_dma: DMA1_CH3, rx_dma: DMA1_CH2) -> Spi<'static, Async> {
        let Self { base, sck, mosi, miso } = self;
        let rx = Spi1RxBuilder { base, sck, miso };
        match mosi {
            Spi1Mosi::PA7(pa7) => { Self::build_rx(rx, pa7, tx_dma, rx_dma) }
            Spi1Mosi::PB5(pb5) => { Self::build_rx(rx, pb5, tx_dma, rx_dma) }
        }
    }

    /// build by rx
    fn build_rx(
        rx: Spi1RxBuilder,
        mosi: impl Peripheral<P=impl MosiPin<SPI1>> + 'static,
        tx_dma: DMA1_CH3,
        rx_dma: DMA1_CH2) -> Spi<'static, Async> {
        let Spi1RxBuilder { base, sck, miso } = rx;
        match miso {
            Spi1Miso::PA6(pa6) => { Self::build_sck(base, sck, mosi, pa6, tx_dma, rx_dma) }
            Spi1Miso::PB4(pb4) => { Self::build_sck(base, sck, mosi, pb4, tx_dma, rx_dma) }
        }
    }

    /// build by sck
    fn build_sck(
        base: SpiBase<SPI1>,
        sck: Spi1Sck,
        mosi: impl Peripheral<P=impl MosiPin<SPI1>> + 'static,
        miso: impl Peripheral<P=impl MisoPin<SPI1>> + 'static,
        tx_dma: DMA1_CH3,
        rx_dma: DMA1_CH2) -> Spi<'static, Async> {
        match sck {
            Spi1Sck::PA5(pa5) => {
                Spi::new(base.spi, pa5, mosi, miso, tx_dma, rx_dma, base.config.unwrap_or_default())
            }
            Spi1Sck::PB3(pb3) => {
                Spi::new(base.spi, pb3, mosi, miso, tx_dma, rx_dma, base.config.unwrap_or_default())
            }
        }
    }

    /// Create a new blocking SPI driver.<br />
    /// more see [Spi::<Blocking>::new_blocking]
    pub fn build_blocking(self) -> Spi<'static, Blocking> {
        let Self { base, sck, mosi, miso } = self;
        let rx = Spi1RxBuilder { base, sck, miso };
        match mosi {
            Spi1Mosi::PA7(pa7) => { Self::build_blocking_rx(rx, pa7) }
            Spi1Mosi::PB5(pb5) => { Self::build_blocking_rx(rx, pb5) }
        }
    }

    /// build blocking by rx
    fn build_blocking_rx(
        rx: Spi1RxBuilder,
        mosi: impl Peripheral<P=impl MosiPin<SPI1>> + 'static) -> Spi<'static, Blocking> {
        let Spi1RxBuilder { base, sck, miso } = rx;
        match miso {
            Spi1Miso::PA6(pa6) => { Self::build_blocking_sck(base, sck, mosi, pa6) }
            Spi1Miso::PB4(pb4) => { Self::build_blocking_sck(base, sck, mosi, pb4) }
        }
    }

    /// build blocking by sck
    fn build_blocking_sck(
        base: SpiBase<SPI1>,
        sck: Spi1Sck,
        mosi: impl Peripheral<P=impl MosiPin<SPI1>> + 'static,
        miso: impl Peripheral<P=impl MisoPin<SPI1>> + 'static) -> Spi<'static, Blocking> {
        match sck {
            Spi1Sck::PA5(pa5) => {
                Spi::new_blocking(base.spi, pa5, mosi, miso, base.config.unwrap_or_default())
            }
            Spi1Sck::PB3(pb3) => {
                Spi::new_blocking(base.spi, pb3, mosi, miso, base.config.unwrap_or_default())
            }
        }
    }
}
