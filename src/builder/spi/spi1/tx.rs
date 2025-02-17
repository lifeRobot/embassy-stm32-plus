use embassy_stm32::mode::{Async, Blocking};
use embassy_stm32::Peripheral;
#[cfg(not(STM32C0))]
use embassy_stm32::peripherals::DMA1_CH3;
use embassy_stm32::peripherals::SPI1;
#[cfg(SPI1_PA12)]
use embassy_stm32::peripherals::PA12;
#[cfg(SPI1_PA2)]
use embassy_stm32::peripherals::PA2;
#[cfg(SPI1_PA7)]
use embassy_stm32::peripherals::PA7;
#[cfg(SPI1_PB5)]
use embassy_stm32::peripherals::PB5;
#[cfg(SPI1_PB6)]
use embassy_stm32::peripherals::PB6;
use embassy_stm32::spi::{Config, SckPin, Spi};
#[cfg(STM32C0)]
use embassy_stm32::spi::TxDma;
use crate::builder::spi::base::SpiBase;
use crate::builder::spi::spi1::Spi1Sck;

/// spi1 mosi pin
pub enum Spi1Mosi {
    #[cfg(SPI1_PA2)]
    PA2(PA2),
    #[cfg(SPI1_PA7)]
    PA7(PA7),
    #[cfg(SPI1_PA12)]
    PA12(PA12),
    #[cfg(SPI1_PB5)]
    PB5(PB5),
    #[cfg(SPI1_PB6)]
    PB6(PB6),
}

/// spi1 tx builder
pub struct Spi1TxBuilder {
    /// spi device
    pub base: SpiBase<SPI1>,
    /// mosi pin
    pub mosi: Spi1Mosi,
}

/// spi1 tx build
macro_rules! spi1_tx_build {
    ($tx_dma:ty) => {
        /// Create a new SPI driver, in TX-only mode (only MOSI pin, no MISO).<br />
        /// more see [`Spi::<Async>::new_txonly`]
        pub fn build(self, sck: Spi1Sck, tx_dma: $tx_dma) -> Spi<'static, Async> {
            match sck {
                #[cfg(SPI1_PA1)]
                Spi1Sck::PA1(pa1) => { self.build_mosi(pa1, tx_dma) }
                #[cfg(SPI1_PA5)]
                Spi1Sck::PA5(pa5) => { self.build_mosi(pa5, tx_dma) }
                #[cfg(SPI1_PB3)]
                Spi1Sck::PB3(pb3) => { self.build_mosi(pb3, tx_dma) }
                #[cfg(SPI1_PB6)]
                Spi1Sck::PB6(pb6) => { self.build_mosi(pb6, tx_dma) }
            }
        }

        /// build by mosi
        fn build_mosi(
            self,
            sck: impl Peripheral<P=impl SckPin<SPI1>> + 'static,
            tx_dma: $tx_dma) -> Spi<'static, Async> {
            match self.mosi {
                #[cfg(SPI1_PA2)]
                Spi1Mosi::PA2(pa2) => { Spi::new_txonly(self.base.spi, sck, pa2, tx_dma, self.base.config.unwrap_or_default()) }
                #[cfg(SPI1_PA7)]
                Spi1Mosi::PA7(pa7) => { Spi::new_txonly(self.base.spi, sck, pa7, tx_dma, self.base.config.unwrap_or_default()) }
                #[cfg(SPI1_PA12)]
                Spi1Mosi::PA12(pa12) => { Spi::new_txonly(self.base.spi, sck, pa12, tx_dma, self.base.config.unwrap_or_default()) }
                #[cfg(SPI1_PB5)]
                Spi1Mosi::PB5(pb5) => { Spi::new_txonly(self.base.spi, sck, pb5, tx_dma, self.base.config.unwrap_or_default()) }
                #[cfg(SPI1_PB6)]
                Spi1Mosi::PB6(pb6) => { Spi::new_txonly(self.base.spi, sck, pb6, tx_dma, self.base.config.unwrap_or_default()) }
            }
        }

        /// Create a new SPI driver, in TX-only mode, without SCK pin.<br />
        /// more see [`Spi::<Async>::new_txonly_nosck`]
        pub fn build_nosck(self, tx_dma: $tx_dma) -> Spi<'static, Async> {
            match self.mosi {
                #[cfg(SPI1_PA2)]
                Spi1Mosi::PA2(pa2) => { Spi::new_txonly_nosck(self.base.spi, pa2, tx_dma, self.base.config.unwrap_or_default()) }
                #[cfg(SPI1_PA7)]
                Spi1Mosi::PA7(pa7) => { Spi::new_txonly_nosck(self.base.spi, pa7, tx_dma, self.base.config.unwrap_or_default()) }
                #[cfg(SPI1_PA12)]
                Spi1Mosi::PA12(pa12) => { Spi::new_txonly_nosck(self.base.spi, pa12, tx_dma, self.base.config.unwrap_or_default()) }
                #[cfg(SPI1_PB5)]
                Spi1Mosi::PB5(pb5) => { Spi::new_txonly_nosck(self.base.spi, pb5, tx_dma, self.base.config.unwrap_or_default()) }
                #[cfg(SPI1_PB6)]
                Spi1Mosi::PB6(pb6) => { Spi::new_txonly_nosck(self.base.spi, pb6, tx_dma, self.base.config.unwrap_or_default()) }
            }
        }
    };
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

    /// Create a new blocking SPI driver, in TX-only mode (only MOSI pin, no MISO).<br />
    /// more see [`Spi::<Blocking>::new_blocking_txonly`]
    pub fn build_blocking(self, sck: Spi1Sck) -> Spi<'static, Blocking> {
        match sck {
            #[cfg(SPI1_PA1)]
            Spi1Sck::PA1(pa1) => { self.build_blocking_mosi(pa1) }
            #[cfg(SPI1_PA5)]
            Spi1Sck::PA5(pa5) => { self.build_blocking_mosi(pa5) }
            #[cfg(SPI1_PB3)]
            Spi1Sck::PB3(pb3) => { self.build_blocking_mosi(pb3) }
            #[cfg(SPI1_PB6)]
            Spi1Sck::PB6(pb6) => { self.build_blocking_mosi(pb6) }
        }
    }

    /// build blocking by mosi
    fn build_blocking_mosi(self, sck: impl Peripheral<P=impl SckPin<SPI1>> + 'static) -> Spi<'static, Blocking> {
        match self.mosi {
            #[cfg(SPI1_PA2)]
            Spi1Mosi::PA2(pa2) => { Spi::new_blocking_txonly(self.base.spi, sck, pa2, self.base.config.unwrap_or_default()) }
            #[cfg(SPI1_PA7)]
            Spi1Mosi::PA7(pa7) => { Spi::new_blocking_txonly(self.base.spi, sck, pa7, self.base.config.unwrap_or_default()) }
            #[cfg(SPI1_PA12)]
            Spi1Mosi::PA12(pa12) => { Spi::new_blocking_txonly(self.base.spi, sck, pa12, self.base.config.unwrap_or_default()) }
            #[cfg(SPI1_PB5)]
            Spi1Mosi::PB5(pb5) => { Spi::new_blocking_txonly(self.base.spi, sck, pb5, self.base.config.unwrap_or_default()) }
            #[cfg(SPI1_PB6)]
            Spi1Mosi::PB6(pb6) => { Spi::new_blocking_txonly(self.base.spi, sck, pb6, self.base.config.unwrap_or_default()) }
        }
    }

    /// Create a new SPI driver, in TX-only mode, without SCK pin.<br />
    /// more see [`Spi::<Blocking>::new_blocking_txonly_nosck`]
    pub fn build_blocking_nosck(self) -> Spi<'static, Blocking> {
        match self.mosi {
            #[cfg(SPI1_PA2)]
            Spi1Mosi::PA2(pa2) => { Spi::new_blocking_txonly_nosck(self.base.spi, pa2, self.base.config.unwrap_or_default()) }
            #[cfg(SPI1_PA7)]
            Spi1Mosi::PA7(pa7) => { Spi::new_blocking_txonly_nosck(self.base.spi, pa7, self.base.config.unwrap_or_default()) }
            #[cfg(SPI1_PA12)]
            Spi1Mosi::PA12(pa12) => { Spi::new_blocking_txonly_nosck(self.base.spi, pa12, self.base.config.unwrap_or_default()) }
            #[cfg(SPI1_PB5)]
            Spi1Mosi::PB5(pb5) => { Spi::new_blocking_txonly_nosck(self.base.spi, pb5, self.base.config.unwrap_or_default()) }
            #[cfg(SPI1_PB6)]
            Spi1Mosi::PB6(pb6) => { Spi::new_blocking_txonly_nosck(self.base.spi, pb6, self.base.config.unwrap_or_default()) }
        }
    }

    #[cfg(STM32C0)]
    spi1_tx_build!(impl Peripheral<P = impl TxDma<SPI1>> + 'static);
    #[cfg(not(STM32C0))]
    spi1_tx_build!(DMA1_CH3);
}
