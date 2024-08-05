/// 60pin gpio
#[macro_export]
macro_rules! f103_gpio_60 {
    () => {
        use embassy_stm32::peripherals::{PA0, PA1, PA10, PA11, PA12, PA13, PA14, PA15, PA3, PA4, PA5, PA6, PA7, PA8, PA9, PB0, PB1, PB10, PB11, PB12, PB13, PB14, PB15, PB2, PB3, PB4, PB5, PB6, PB7, PB8, PB9, PC0, PC1, PC10, PC11, PC12, PC13, PC14, PC15, PC2, PC3, PC4, PC5, PC6, PC7, PC8, PC9, PD0, PD1, PD10, PD11, PD12, PD13, PD14, PD15, PD2, PD3, PD4, PD5, PD6, PD7, PD8, PD9};
        use $crate::traits::gpio::input::GpioInput;
        use $crate::traits::gpio::output::GpioOutput;

        $crate::impl_gpio_trait!(PA0);
        $crate::impl_gpio_trait!(PA1);
        $crate::impl_gpio_trait!(PA3);
        $crate::impl_gpio_trait!(PA4);
        $crate::impl_gpio_trait!(PA5);
        $crate::impl_gpio_trait!(PA6);
        $crate::impl_gpio_trait!(PA7);
        $crate::impl_gpio_trait!(PA8);
        $crate::impl_gpio_trait!(PA9);
        $crate::impl_gpio_trait!(PA10);
        $crate::impl_gpio_trait!(PA11);
        $crate::impl_gpio_trait!(PA12);
        $crate::impl_gpio_trait!(PA13);
        $crate::impl_gpio_trait!(PA14);
        $crate::impl_gpio_trait!(PA15);
        $crate::impl_gpio_trait!(PB0);
        $crate::impl_gpio_trait!(PB1);
        $crate::impl_gpio_trait!(PB2);
        $crate::impl_gpio_trait!(PB3);
        $crate::impl_gpio_trait!(PB4);
        $crate::impl_gpio_trait!(PB5);
        $crate::impl_gpio_trait!(PB6);
        $crate::impl_gpio_trait!(PB7);
        $crate::impl_gpio_trait!(PB8);
        $crate::impl_gpio_trait!(PB9);
        $crate::impl_gpio_trait!(PB10);
        $crate::impl_gpio_trait!(PB11);
        $crate::impl_gpio_trait!(PB12);
        $crate::impl_gpio_trait!(PB13);
        $crate::impl_gpio_trait!(PB14);
        $crate::impl_gpio_trait!(PB15);
        $crate::impl_gpio_trait!(PC0);
        $crate::impl_gpio_trait!(PC1);
        $crate::impl_gpio_trait!(PC2);
        $crate::impl_gpio_trait!(PC3);
        $crate::impl_gpio_trait!(PC4);
        $crate::impl_gpio_trait!(PC5);
        $crate::impl_gpio_trait!(PC6);
        $crate::impl_gpio_trait!(PC7);
        $crate::impl_gpio_trait!(PC8);
        $crate::impl_gpio_trait!(PC9);
        $crate::impl_gpio_trait!(PC10);
        $crate::impl_gpio_trait!(PC11);
        $crate::impl_gpio_trait!(PC12);
        $crate::impl_gpio_trait!(PC13);
        $crate::impl_gpio_trait!(PC14);
        $crate::impl_gpio_trait!(PC15);
        $crate::impl_gpio_trait!(PD0);
        $crate::impl_gpio_trait!(PD1);
        $crate::impl_gpio_trait!(PD2);
        $crate::impl_gpio_trait!(PD3);
        $crate::impl_gpio_trait!(PD4);
        $crate::impl_gpio_trait!(PD5);
        $crate::impl_gpio_trait!(PD6);
        $crate::impl_gpio_trait!(PD7);
        $crate::impl_gpio_trait!(PD8);
        $crate::impl_gpio_trait!(PD9);
        $crate::impl_gpio_trait!(PD10);
        $crate::impl_gpio_trait!(PD11);
        $crate::impl_gpio_trait!(PD12);
        $crate::impl_gpio_trait!(PD13);
        $crate::impl_gpio_trait!(PD14);
        $crate::impl_gpio_trait!(PD15);
    };
}

/// 75in gpio
#[macro_export]
macro_rules! f103_gpio_75 {
    () => {
        $crate::f103_gpio_60!();
        use embassy_stm32::peripherals::{PE0, PE1, PE10, PE11, PE12, PE13, PE14, PE15, PE2, PE3, PE4, PE5, PE6, PE7, PE8, PE9};

        $crate::impl_gpio_trait!(PE0);
        $crate::impl_gpio_trait!(PE1);
        $crate::impl_gpio_trait!(PE2);
        $crate::impl_gpio_trait!(PE3);
        $crate::impl_gpio_trait!(PE4);
        $crate::impl_gpio_trait!(PE5);
        $crate::impl_gpio_trait!(PE6);
        $crate::impl_gpio_trait!(PE7);
        $crate::impl_gpio_trait!(PE8);
        $crate::impl_gpio_trait!(PE9);
        $crate::impl_gpio_trait!(PE10);
        $crate::impl_gpio_trait!(PE11);
        $crate::impl_gpio_trait!(PE12);
        $crate::impl_gpio_trait!(PE13);
        $crate::impl_gpio_trait!(PE14);
        $crate::impl_gpio_trait!(PE15);
    };
}

/// 105pin gpio
#[macro_export]
macro_rules! f103_gpio_105 {
    () => {
        $crate::f103_gpio_75!();
        use embassy_stm32::peripherals::{PF0, PF1, PF10, PF11, PF12, PF13, PF14, PF15, PF2, PF3, PF4, PF5, PF6, PF7, PF8, PF9, PG0, PG1, PG10, PG11, PG12, PG13, PG14, PG15, PG2, PG3, PG4, PG5, PG6, PG7, PG8, PG9};

        $crate::impl_gpio_trait!(PF0);
        $crate::impl_gpio_trait!(PF1);
        $crate::impl_gpio_trait!(PF2);
        $crate::impl_gpio_trait!(PF3);
        $crate::impl_gpio_trait!(PF4);
        $crate::impl_gpio_trait!(PF5);
        $crate::impl_gpio_trait!(PF6);
        $crate::impl_gpio_trait!(PF7);
        $crate::impl_gpio_trait!(PF8);
        $crate::impl_gpio_trait!(PF9);
        $crate::impl_gpio_trait!(PF10);
        $crate::impl_gpio_trait!(PF11);
        $crate::impl_gpio_trait!(PF12);
        $crate::impl_gpio_trait!(PF13);
        $crate::impl_gpio_trait!(PF14);
        $crate::impl_gpio_trait!(PF15);
        $crate::impl_gpio_trait!(PG0);
        $crate::impl_gpio_trait!(PG1);
        $crate::impl_gpio_trait!(PG2);
        $crate::impl_gpio_trait!(PG3);
        $crate::impl_gpio_trait!(PG4);
        $crate::impl_gpio_trait!(PG5);
        $crate::impl_gpio_trait!(PG6);
        $crate::impl_gpio_trait!(PG7);
        $crate::impl_gpio_trait!(PG8);
        $crate::impl_gpio_trait!(PG9);
        $crate::impl_gpio_trait!(PG10);
        $crate::impl_gpio_trait!(PG11);
        $crate::impl_gpio_trait!(PG12);
        $crate::impl_gpio_trait!(PG13);
        $crate::impl_gpio_trait!(PG14);
        $crate::impl_gpio_trait!(PG15);
    };
}
