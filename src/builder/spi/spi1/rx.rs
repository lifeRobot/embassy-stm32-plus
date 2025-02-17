use embassy_stm32::mode::{Async, Blocking};
use embassy_stm32::Peripheral;
#[cfg(not(STM32C0))]
use embassy_stm32::peripherals::{DMA1_CH2, DMA1_CH3};
use embassy_stm32::peripherals::SPI1;
#[cfg(SPI1_PA11)]
use embassy_stm32::peripherals::PA11;
#[cfg(SPI1_PA6)]
use embassy_stm32::peripherals::PA6;
#[cfg(SPI1_PB4)]
use embassy_stm32::peripherals::PB4;
#[cfg(SPI1_PB6)]
use embassy_stm32::peripherals::PB6;
use embassy_stm32::spi::{Config, SckPin, Spi};
#[cfg(STM32C0)]
use embassy_stm32::spi::{RxDma, TxDma};
use crate::builder::spi::base::SpiBase;
use crate::builder::spi::spi1::Spi1Sck;

/// spi1 miso pin
pub enum Spi1Miso {
    #[cfg(SPI1_PA6)]
    PA6(PA6),
    #[cfg(SPI1_PA11)]
    PA11(PA11),
    #[cfg(SPI1_PB4)]
    PB4(PB4),
    #[cfg(SPI1_PB6)]
    PB6(PB6),
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

/// spi1 rx build
macro_rules! spi1_rx_build {
    ($tx_dma:ty,$rx_dma:ty) => {
        /// Create a new SPI driver, in RX-only mode (only MISO pin, no MOSI).<br />
        /// more see [`Spi::<Async>::new_rxonly`]
        pub fn build(self, tx_dma: $tx_dma, rx_dma: $rx_dma) -> Spi<'static, Async> {
            let Self { base, sck, miso } = self;
            match sck {
                #[cfg(SPI1_PA1)]
                Spi1Sck::PA1(pa1) => { Self::build_miso(base, pa1, miso, tx_dma, rx_dma) }
                #[cfg(SPI1_PA5)]
                Spi1Sck::PA5(pa5) => { Self::build_miso(base, pa5, miso, tx_dma, rx_dma) }
                #[cfg(SPI1_PB3)]
                Spi1Sck::PB3(pb3) => { Self::build_miso(base, pb3, miso, tx_dma, rx_dma) }
                #[cfg(SPI1_PB6)]
                Spi1Sck::PB6(pb6) => { Self::build_miso(base, pb6, miso, tx_dma, rx_dma) }
            }
        }

        /// build by miso
        fn build_miso(
            base: SpiBase<SPI1>,
            sck: impl Peripheral<P=impl SckPin<SPI1>> + 'static,
            miso: Spi1Miso,
            tx_dma: $tx_dma,
            rx_dma: $rx_dma) -> Spi<'static, Async> {
            match miso {
                #[cfg(SPI1_PA6)]
                Spi1Miso::PA6(pa6) => { Spi::new_rxonly(base.spi, sck, pa6, tx_dma, rx_dma, base.config.unwrap_or_default()) }
                #[cfg(SPI1_PA11)]
                Spi1Miso::PA11(pa11) => { Spi::new_rxonly(base.spi, sck, pa11, tx_dma, rx_dma, base.config.unwrap_or_default()) }
                #[cfg(SPI1_PB4)]
                Spi1Miso::PB4(pb4) => { Spi::new_rxonly(base.spi, sck, pb4, tx_dma, rx_dma, base.config.unwrap_or_default()) }
                #[cfg(SPI1_PB6)]
                Spi1Miso::PB6(pb6) => { Spi::new_rxonly(base.spi, sck, pb6, tx_dma, rx_dma, base.config.unwrap_or_default()) }
            }
        }
    };
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

    /// Create a new blocking SPI driver, in RX-only mode (only MISO pin, no MOSI).<br />
    /// more see [`Spi::<Blocking>::new_blocking_rxonly`]
    pub fn build_blocking(self) -> Spi<'static, Blocking> {
        let Self { base, sck, miso } = self;
        match sck {
            #[cfg(SPI1_PA1)]
            Spi1Sck::PA1(pa1) => { Self::build_blocking_miso(base, pa1, miso) }
            #[cfg(SPI1_PA5)]
            Spi1Sck::PA5(pa5) => { Self::build_blocking_miso(base, pa5, miso) }
            #[cfg(SPI1_PB3)]
            Spi1Sck::PB3(pb3) => { Self::build_blocking_miso(base, pb3, miso) }
            #[cfg(SPI1_PB6)]
            Spi1Sck::PB6(pb6) => { Self::build_blocking_miso(base, pb6, miso) }
        }
    }

    /// build blocking by miso
    fn build_blocking_miso(
        base: SpiBase<SPI1>,
        sck: impl Peripheral<P=impl SckPin<SPI1>> + 'static,
        miso: Spi1Miso) -> Spi<'static, Blocking> {
        match miso {
            #[cfg(SPI1_PA6)]
            Spi1Miso::PA6(pa6) => { Spi::new_blocking_rxonly(base.spi, sck, pa6, base.config.unwrap_or_default()) }
            #[cfg(SPI1_PA11)]
            Spi1Miso::PA11(pa11) => { Spi::new_blocking_rxonly(base.spi, sck, pa11, base.config.unwrap_or_default()) }
            #[cfg(SPI1_PB4)]
            Spi1Miso::PB4(pb4) => { Spi::new_blocking_rxonly(base.spi, sck, pb4, base.config.unwrap_or_default()) }
            #[cfg(SPI1_PB6)]
            Spi1Miso::PB6(pb6) => { Spi::new_blocking_rxonly(base.spi, sck, pb6, base.config.unwrap_or_default()) }
        }
    }

    #[cfg(STM32C0)]
    spi1_rx_build!(impl Peripheral<P = impl TxDma<SPI1>> + 'static,impl Peripheral<P = impl RxDma<SPI1>> + 'static);
    #[cfg(not(STM32C0))]
    spi1_rx_build!(DMA1_CH3,DMA1_CH2);
}