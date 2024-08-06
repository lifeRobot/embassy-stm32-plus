/// f103 i2c2
#[macro_export]
macro_rules! f103_i2c2 {
    () => {
        pub mod i2c2 {
            use embassy_stm32::{bind_interrupts, i2c};
            use embassy_stm32::peripherals::{DMA1_CH4, DMA1_CH5, I2C2, PB10, PB11};

            bind_interrupts!(pub struct Irqs {
                I2C2_ER => i2c::ErrorInterruptHandler<I2C2>;
                I2C2_EV => i2c::EventInterruptHandler<I2C2>;
            });

            crate::use_i2c_trait!();
            crate::impl_i2c_trait!(I2C2,PB10,PB11);
            crate::impl_i2c_trait!(I2C2,PB10,PB11,DMA1_CH4,DMA1_CH5);
        }
    };
}

