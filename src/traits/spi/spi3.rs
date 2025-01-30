use embassy_stm32::peripherals::{PB3, PB4, PB5, SPI3};
use crate::builder::spi::spi3::rx::Spi3RxBuilder;
use crate::builder::spi::spi3::Spi3Builder;
use crate::builder::spi::spi3::tx::Spi3TxBuilder;

/// spi3 trait
pub trait Spi3Trait {
    /// create builder
    fn builder(self, sck: PB3, mosi: PB5, miso: PB4) -> Spi3Builder;

    /// create tx builder
    fn tx_builder(self, mosi: PB5) -> Spi3TxBuilder;

    /// create rx builder
    fn rx_builder(self, sck: PB3, miso: PB4) -> Spi3RxBuilder;
}

/// SPI3 support spi3 trait
impl Spi3Trait for SPI3 {
    #[inline]
    fn builder(self, sck: PB3, mosi: PB5, miso: PB4) -> Spi3Builder {
        Spi3Builder::new(self, sck, mosi, miso)
    }

    #[inline]
    fn tx_builder(self, mosi: PB5) -> Spi3TxBuilder {
        Spi3TxBuilder::new(self, mosi)
    }

    #[inline]
    fn rx_builder(self, sck: PB3, miso: PB4) -> Spi3RxBuilder {
        Spi3RxBuilder::new(self, sck, miso)
    }
}
