/// f103 i2c1 base
#[macro_export]
macro_rules! f103_i2c1 {
    () => {
        use embassy_stm32::{bind_interrupts, i2c};
        use embassy_stm32::peripherals::{DMA1_CH6, DMA1_CH7, I2C1, PB6, PB7};

        bind_interrupts!(pub struct Irqs {
            I2C1_ER => i2c::ErrorInterruptHandler<I2C1>;
            I2C1_EV => i2c::EventInterruptHandler<I2C1>;
        });

        crate::use_i2c_trait!();
        crate::impl_i2c_trait!(I2C1,PB6,PB7);
        crate::impl_i2c_trait!(I2C1,PB6,PB7,DMA1_CH6,DMA1_CH7);
    };
}

/// f103tx i2c1
#[macro_export]
macro_rules! f103t_i2c1 {
    () => {
        pub mod i2c1 {
            $crate::f103_i2c1!();
        }
    };
}

/// f103cx i2c1
#[macro_export]
macro_rules! f103c_i2c1 {
    () => {
        pub mod i2c1 {
            $crate::f103_i2c1!();
            use embassy_stm32::peripherals::{PB8, PB9};

            crate::impl_i2c_trait!(I2C1,PB6,PB9);
            crate::impl_i2c_trait!(I2C1,PB8,PB7);
            crate::impl_i2c_trait!(I2C1,PB8,PB9);

            crate::impl_i2c_trait!(I2C1,PB6,PB9,DMA1_CH6,DMA1_CH7);
            crate::impl_i2c_trait!(I2C1,PB8,PB7,DMA1_CH6,DMA1_CH7);
            crate::impl_i2c_trait!(I2C1,PB8,PB9,DMA1_CH6,DMA1_CH7);
        }
    };
}
