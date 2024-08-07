/// f103 spi2
#[macro_export]
macro_rules! mod_spi2 {
    () => {
        pub mod spi2 {
            use embassy_stm32::peripherals::{DMA1_CH4, DMA1_CH5, PB13, PB14, PB15, SPI2};

            crate::use_spi_trait!();
            crate::impl_spi_trait!(SPI2,PB13,PB15,PB14);
            crate::impl_spi_trait!(SPI2,PB13,PB15,PB14,DMA1_CH5,DMA1_CH4);
        }
    };
}

