/// f103 spi1
#[macro_export]
macro_rules! mod_spi1 {
    () => {
        pub mod spi1 {
            use embassy_stm32::peripherals::{DMA1_CH2, DMA1_CH3, PA5, PA6, PA7, PB3, PB4, PB5, SPI1};

            crate::use_spi_trait!();

            crate::impl_spi_trait!(SPI1,PA5,PA7,PA6);
            crate::impl_spi_trait!(SPI1,PA5,PA7,PB4);
            crate::impl_spi_trait!(SPI1,PA5,PB5,PA6);
            crate::impl_spi_trait!(SPI1,PA5,PB5,PB4);
            crate::impl_spi_trait!(SPI1,PB3,PA7,PA6);
            crate::impl_spi_trait!(SPI1,PB3,PA7,PB4);
            crate::impl_spi_trait!(SPI1,PB3,PB5,PA6);
            crate::impl_spi_trait!(SPI1,PB3,PB5,PB4);

            crate::impl_spi_trait!(SPI1,PA5,PA7,PA6,DMA1_CH3,DMA1_CH2);
            crate::impl_spi_trait!(SPI1,PA5,PA7,PB4,DMA1_CH3,DMA1_CH2);
            crate::impl_spi_trait!(SPI1,PA5,PB5,PA6,DMA1_CH3,DMA1_CH2);
            crate::impl_spi_trait!(SPI1,PA5,PB5,PB4,DMA1_CH3,DMA1_CH2);
            crate::impl_spi_trait!(SPI1,PB3,PA7,PA6,DMA1_CH3,DMA1_CH2);
            crate::impl_spi_trait!(SPI1,PB3,PA7,PB4,DMA1_CH3,DMA1_CH2);
            crate::impl_spi_trait!(SPI1,PB3,PB5,PA6,DMA1_CH3,DMA1_CH2);
            crate::impl_spi_trait!(SPI1,PB3,PB5,PB4,DMA1_CH3,DMA1_CH2);
        }
    };
}

