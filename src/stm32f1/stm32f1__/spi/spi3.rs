/// f103 spi3
#[macro_export]
macro_rules! mod_spi3 {
    () => {
        pub mod spi3 {
            use embassy_stm32::peripherals::{DMA2_CH1, DMA2_CH2, PB3, PB4, PB5, SPI3};

            crate::use_spi_trait!();
            crate::impl_spi_trait!(SPI3,PB3,PB5,PB4);
            crate::impl_spi_trait!(SPI3,PB3,PB5,PB4,DMA2_CH2,DMA2_CH1);
        }
    };
}

