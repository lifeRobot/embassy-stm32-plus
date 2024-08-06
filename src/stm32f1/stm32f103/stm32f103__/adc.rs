/// support adc1
#[macro_export]
macro_rules! impl_adc1 {
    () => {
        use embassy_stm32::peripherals::ADC1;
        use crate::traits::adc::AdcTrait;

        impl AdcTrait for ADC1 {}
    };
}

/// support adc2
#[macro_export]
macro_rules! impl_adc2 {
    () => {
        $crate::impl_adc1!();
        use embassy_stm32::peripherals::ADC2;

        impl AdcTrait for ADC2 {}
    };
}

/// support adc3
#[macro_export]
macro_rules! impl_adc3 {
    () => {
        $crate::impl_adc2!();
        use embassy_stm32::peripherals::ADC3;

        impl AdcTrait for ADC3 {}
    };
}
