use embassy_stm32::mode::{Async, Blocking};
use embassy_stm32::Peripheral;
use embassy_stm32::peripherals::{DMA2_CH1, DMA2_CH2, PB3, SPI3};
#[cfg(SPI3_PC10)]
use embassy_stm32::peripherals::PC10;
use embassy_stm32::spi::{Config, MisoPin, MosiPin, Spi};
use crate::builder::spi::base::SpiBase;
use crate::builder::spi::spi3::rx::{Spi3Miso, Spi3RxBuilder};
use crate::builder::spi::spi3::tx::Spi3Mosi;

pub mod rx;
pub mod tx;

/// spi3 sck pin
pub enum Spi3Sck {
    PB3(PB3),
    #[cfg(SPI3_PC10)]
    PC10(PC10),
}

/// spi3 builder
pub struct Spi3Builder {
    /// spi device
    pub base: SpiBase<SPI3>,
    /// sck pin
    pub sck: Spi3Sck,
    /// mosi pin
    pub mosi: Spi3Mosi,
    /// miso pin
    pub miso: Spi3Miso,
}

/// custom method
impl Spi3Builder {
    /// create builder
    #[inline]
    pub fn new(spi: SPI3, sck: Spi3Sck, mosi: Spi3Mosi, miso: Spi3Miso) -> Self {
        Self { base: SpiBase::new(spi), sck, mosi, miso }
    }

    /// set spi config
    #[inline]
    pub fn config(mut self, config: Config) -> Self {
        self.base.set_config(config);
        self
    }

    /// Create a new SPI driver.<br />
    /// more see [`Spi::<Async>::new`]
    #[inline]
    pub fn build(self, tx_dma: DMA2_CH2, rx_dma: DMA2_CH1) -> Spi<'static, Async> {
        let Self { base, sck, mosi, miso } = self;
        let rx = Spi3RxBuilder { base, sck, miso };
        match mosi {
            Spi3Mosi::PB5(pb5) => { Self::build_rx(rx, pb5, tx_dma, rx_dma) }
            #[cfg(SPI3_PC12)]
            Spi3Mosi::PC12(pc12) => { Self::build_rx(rx, pc12, tx_dma, rx_dma) }
        }
    }

    /// build by rx
    fn build_rx(
        rx: Spi3RxBuilder,
        mosi: impl Peripheral<P=impl MosiPin<SPI3>> + 'static,
        tx_dma: DMA2_CH2,
        rx_dma: DMA2_CH1) -> Spi<'static, Async> {
        let Spi3RxBuilder { base, sck, miso } = rx;
        match miso {
            Spi3Miso::PB4(pb4) => { Self::build_sck(base, sck, mosi, pb4, tx_dma, rx_dma) }
            #[cfg(SPI3_PC11)]
            Spi3Miso::PC11(pc11) => { Self::build_sck(base, sck, mosi, pc11, tx_dma, rx_dma) }
        }
    }

    /// build by sck
    fn build_sck(
        base: SpiBase<SPI3>,
        sck: Spi3Sck,
        mosi: impl Peripheral<P=impl MosiPin<SPI3>> + 'static,
        miso: impl Peripheral<P=impl MisoPin<SPI3>> + 'static,
        tx_dma: DMA2_CH2,
        rx_dma: DMA2_CH1) -> Spi<'static, Async> {
        match sck {
            Spi3Sck::PB3(pb3) => { Spi::new(base.spi, pb3, mosi, miso, tx_dma, rx_dma, base.config.unwrap_or_default()) }
            #[cfg(SPI3_PC10)]
            Spi3Sck::PC10(pc10) => { Spi::new(base.spi, pc10, mosi, miso, tx_dma, rx_dma, base.config.unwrap_or_default()) }
        }
    }

    /// Create a new blocking SPI driver.<br />
    /// more see [`Spi::<Blocking>::new_blocking`]
    #[inline]
    pub fn build_blocking(self) -> Spi<'static, Blocking> {
        let Self { base, sck, mosi, miso } = self;
        let rx = Spi3RxBuilder { base, sck, miso };
        match mosi {
            Spi3Mosi::PB5(pb5) => { Self::build_blocking_rx(rx, pb5) }
            #[cfg(SPI3_PC12)]
            Spi3Mosi::PC12(pc12) => { Self::build_blocking_rx(rx, pc12) }
        }
    }

    /// build blocking by rx
    fn build_blocking_rx(rx: Spi3RxBuilder, mosi: impl Peripheral<P=impl MosiPin<SPI3>> + 'static) -> Spi<'static, Blocking> {
        let Spi3RxBuilder { base, sck, miso } = rx;
        match miso {
            Spi3Miso::PB4(pb4) => { Self::build_blocking_sck(base, sck, mosi, pb4) }
            #[cfg(SPI3_PC11)]
            Spi3Miso::PC11(pc11) => { Self::build_blocking_sck(base, sck, mosi, pc11) }
        }
    }

    /// build blocking by sck
    fn build_blocking_sck(
        base: SpiBase<SPI3>,
        sck: Spi3Sck,
        mosi: impl Peripheral<P=impl MosiPin<SPI3>> + 'static,
        miso: impl Peripheral<P=impl MisoPin<SPI3>> + 'static) -> Spi<'static, Blocking> {
        match sck {
            Spi3Sck::PB3(pb3) => { Spi::new_blocking(base.spi, pb3, mosi, miso, base.config.unwrap_or_default()) }
            #[cfg(SPI3_PC10)]
            Spi3Sck::PC10(pc10) => { Spi::new_blocking(base.spi, pc10, mosi, miso, base.config.unwrap_or_default()) }
        }
    }
}
