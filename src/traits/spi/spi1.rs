use embassy_stm32::peripherals::SPI1;
use crate::builder::spi::spi1::rx::{Spi1Miso, Spi1RxBuilder};
use crate::builder::spi::spi1::{Spi1Builder, Spi1Sck};
use crate::builder::spi::spi1::tx::{Spi1Mosi, Spi1TxBuilder};

/// spi1 trait
pub trait Spi1Trait {
    /// create builder
    fn builder(self, sck: Spi1Sck, mosi: Spi1Mosi, miso: Spi1Miso) -> Spi1Builder;

    /// create tx builder
    fn tx_builder(self, mosi: Spi1Mosi) -> Spi1TxBuilder;

    /// create rx builder
    fn rx_builder(self, sck: Spi1Sck, miso: Spi1Miso) -> Spi1RxBuilder;
}

/// SPI1 support spi1 trait
impl Spi1Trait for SPI1 {
    #[inline]
    fn builder(self, sck: Spi1Sck, mosi: Spi1Mosi, miso: Spi1Miso) -> Spi1Builder {
        Spi1Builder::new(self, sck, mosi, miso)
    }

    #[inline]
    fn tx_builder(self, mosi: Spi1Mosi) -> Spi1TxBuilder {
        Spi1TxBuilder::new(self, mosi)
    }

    #[inline]
    fn rx_builder(self, sck: Spi1Sck, miso: Spi1Miso) -> Spi1RxBuilder {
        Spi1RxBuilder::new(self, sck, miso)
    }
}
