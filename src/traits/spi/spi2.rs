use embassy_stm32::peripherals::{PB13, PB14, PB15, SPI2};
use crate::builder::spi::spi2::rx::Spi2RxBuilder;
use crate::builder::spi::spi2::Spi2Builder;
use crate::builder::spi::spi2::tx::Spi2TxBuilder;

/// spi2 trait
pub trait Spi2Trait {
    /// create builder
    fn builder(self, sck: PB13, mosi: PB15, miso: PB14) -> Spi2Builder;

    /// create tx builder
    fn tx_builder(self, mosi: PB15) -> Spi2TxBuilder;

    /// create rx builder
    fn rx_builder(self, sck: PB13, miso: PB14) -> Spi2RxBuilder;
}

/// SPI2 support spi2 trait
impl Spi2Trait for SPI2 {
    #[inline]
    fn builder(self, sck: PB13, mosi: PB15, miso: PB14) -> Spi2Builder {
        Spi2Builder::new(self, sck, mosi, miso)
    }

    #[inline]
    fn tx_builder(self, mosi: PB15) -> Spi2TxBuilder {
        Spi2TxBuilder::new(self, mosi)
    }

    #[inline]
    fn rx_builder(self, sck: PB13, miso: PB14) -> Spi2RxBuilder {
        Spi2RxBuilder::new(self, sck, miso)
    }
}
