use embassy_stm32::mode::{Async, Blocking};
use embassy_stm32::Peripheral;
#[cfg(not(STM32C0))]
use embassy_stm32::peripherals::{DMA1_CH2, DMA1_CH3};
use embassy_stm32::peripherals::SPI1;
#[cfg(SPI1_PA1)]
use embassy_stm32::peripherals::PA1;
#[cfg(SPI1_PA5)]
use embassy_stm32::peripherals::PA5;
#[cfg(SPI1_PB3)]
use embassy_stm32::peripherals::PB3;
#[cfg(SPI1_PB6)]
use embassy_stm32::peripherals::PB6;
use embassy_stm32::spi::{Config, MisoPin, MosiPin, Spi};
#[cfg(STM32C0)]
use embassy_stm32::spi::{RxDma, TxDma};
use crate::builder::spi::base::SpiBase;
use crate::builder::spi::spi1::rx::{Spi1Miso, Spi1RxBuilder};
use crate::builder::spi::spi1::tx::Spi1Mosi;

pub mod rx;
pub mod tx;

/// spi1 sck pin
pub enum Spi1Sck {
    #[cfg(SPI1_PA1)]
    PA1(PA1),
    #[cfg(SPI1_PA5)]
    PA5(PA5),
    #[cfg(SPI1_PB3)]
    PB3(PB3),
    #[cfg(SPI1_PB6)]
    PB6(PB6),
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

/// spi1 build
macro_rules! spi1_build {
    ($tx_dma:ty,$rx_dma:ty) => {
        /// Create a new SPI driver.<br />
        /// more see [`Spi::<Async>::new`]
        pub fn build(self, tx_dma: $tx_dma, rx_dma: $rx_dma) -> Spi<'static, Async> {
            let Self { base, sck, mosi, miso } = self;
            let rx = Spi1RxBuilder { base, sck, miso };
            match mosi {
                #[cfg(SPI1_PA2)]
                Spi1Mosi::PA2(pa2) => { Self::build_rx(rx, pa2, tx_dma, rx_dma) }
                #[cfg(SPI1_PA7)]
                Spi1Mosi::PA7(pa7) => { Self::build_rx(rx, pa7, tx_dma, rx_dma) }
                #[cfg(SPI1_PA12)]
                Spi1Mosi::PA12(pa12) => { Self::build_rx(rx, pa12, tx_dma, rx_dma) }
                #[cfg(SPI1_PB5)]
                Spi1Mosi::PB5(pb5) => { Self::build_rx(rx, pb5, tx_dma, rx_dma) }
                #[cfg(SPI1_PB6)]
                Spi1Mosi::PB6(pb6) => { Self::build_rx(rx, pb6, tx_dma, rx_dma) }
            }
        }

        /// build by rx
        fn build_rx(
            rx: Spi1RxBuilder,
            mosi: impl Peripheral<P=impl MosiPin<SPI1>> + 'static,
            tx_dma: $tx_dma,
            rx_dma: $rx_dma) -> Spi<'static, Async> {
            let Spi1RxBuilder { base, sck, miso } = rx;
            match miso {
                #[cfg(SPI1_PA6)]
                Spi1Miso::PA6(pa6) => { Self::build_sck(base, sck, mosi, pa6, tx_dma, rx_dma) }
                #[cfg(SPI1_PA11)]
                Spi1Miso::PA11(pa11) => { Self::build_sck(base, sck, mosi, pa11, tx_dma, rx_dma) }
                #[cfg(SPI1_PB4)]
                Spi1Miso::PB4(pb4) => { Self::build_sck(base, sck, mosi, pb4, tx_dma, rx_dma) }
                #[cfg(SPI1_PB6)]
                Spi1Miso::PB6(pb6) => { Self::build_sck(base, sck, mosi, pb6, tx_dma, rx_dma) }
            }
        }

        /// build by sck
        fn build_sck(
            base: SpiBase<SPI1>,
            sck: Spi1Sck,
            mosi: impl Peripheral<P=impl MosiPin<SPI1>> + 'static,
            miso: impl Peripheral<P=impl MisoPin<SPI1>> + 'static,
            tx_dma: $tx_dma,
            rx_dma: $rx_dma) -> Spi<'static, Async> {
            match sck {
                #[cfg(SPI1_PA1)]
                Spi1Sck::PA1(pa1) => { Spi::new(base.spi, pa1, mosi, miso, tx_dma, rx_dma, base.config.unwrap_or_default()) }
                #[cfg(SPI1_PA5)]
                Spi1Sck::PA5(pa5) => { Spi::new(base.spi, pa5, mosi, miso, tx_dma, rx_dma, base.config.unwrap_or_default()) }
                #[cfg(SPI1_PB3)]
                Spi1Sck::PB3(pb3) => { Spi::new(base.spi, pb3, mosi, miso, tx_dma, rx_dma, base.config.unwrap_or_default()) }
                #[cfg(SPI1_PB6)]
                Spi1Sck::PB6(pb6) => { Spi::new(base.spi, pb6, mosi, miso, tx_dma, rx_dma, base.config.unwrap_or_default()) }
            }
        }
    };
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

    /// Create a new blocking SPI driver.<br />
    /// more see [`Spi::<Blocking>::new_blocking`]
    pub fn build_blocking(self) -> Spi<'static, Blocking> {
        let Self { base, sck, mosi, miso } = self;
        let rx = Spi1RxBuilder { base, sck, miso };
        match mosi {
            #[cfg(SPI1_PA2)]
            Spi1Mosi::PA2(pa2) => { Self::build_blocking_rx(rx, pa2) }
            #[cfg(SPI1_PA7)]
            Spi1Mosi::PA7(pa7) => { Self::build_blocking_rx(rx, pa7) }
            #[cfg(SPI1_PA12)]
            Spi1Mosi::PA12(pa12) => { Self::build_blocking_rx(rx, pa12) }
            #[cfg(SPI1_PB5)]
            Spi1Mosi::PB5(pb5) => { Self::build_blocking_rx(rx, pb5) }
            #[cfg(SPI1_PB6)]
            Spi1Mosi::PB6(pb6) => { Self::build_blocking_rx(rx, pb6) }
        }
    }

    /// build blocking by rx
    fn build_blocking_rx(rx: Spi1RxBuilder, mosi: impl Peripheral<P=impl MosiPin<SPI1>> + 'static) -> Spi<'static, Blocking> {
        let Spi1RxBuilder { base, sck, miso } = rx;
        match miso {
            #[cfg(SPI1_PA6)]
            Spi1Miso::PA6(pa6) => { Self::build_blocking_sck(base, sck, mosi, pa6) }
            #[cfg(SPI1_PA11)]
            Spi1Miso::PA11(pa11) => { Self::build_blocking_sck(base, sck, mosi, pa11) }
            #[cfg(SPI1_PB4)]
            Spi1Miso::PB4(pb4) => { Self::build_blocking_sck(base, sck, mosi, pb4) }
            #[cfg(SPI1_PB6)]
            Spi1Miso::PB6(pb6) => { Self::build_blocking_sck(base, sck, mosi, pb6) }
        }
    }

    /// build blocking by sck
    fn build_blocking_sck(
        base: SpiBase<SPI1>,
        sck: Spi1Sck,
        mosi: impl Peripheral<P=impl MosiPin<SPI1>> + 'static,
        miso: impl Peripheral<P=impl MisoPin<SPI1>> + 'static) -> Spi<'static, Blocking> {
        match sck {
            #[cfg(SPI1_PA1)]
            Spi1Sck::PA1(pa1) => { Spi::new_blocking(base.spi, pa1, mosi, miso, base.config.unwrap_or_default()) }
            #[cfg(SPI1_PA5)]
            Spi1Sck::PA5(pa5) => { Spi::new_blocking(base.spi, pa5, mosi, miso, base.config.unwrap_or_default()) }
            #[cfg(SPI1_PB3)]
            Spi1Sck::PB3(pb3) => { Spi::new_blocking(base.spi, pb3, mosi, miso, base.config.unwrap_or_default()) }
            #[cfg(SPI1_PB6)]
            Spi1Sck::PB6(pb6) => { Spi::new_blocking(base.spi, pb6, mosi, miso, base.config.unwrap_or_default()) }
        }
    }

    #[cfg(STM32C0)]
    spi1_build!(impl Peripheral<P = impl TxDma<SPI1>> + 'static,impl Peripheral<P = impl RxDma<SPI1>> + 'static);
    #[cfg(not(STM32C0))]
    spi1_build!(DMA1_CH3,DMA1_CH2);
}
