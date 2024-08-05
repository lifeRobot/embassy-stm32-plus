use embassy_stm32::gpio::{Input, Pin, Pull};

/// gpio input trait
pub trait GpioInput: Pin {
    fn input(self) -> Input<'static, Self> {
        self.input_with_pull(Pull::None)
    }

    fn input_with_pull(self, pull: Pull) -> Input<'static, Self> {
        Input::new(self, pull)
    }
}

/// impl gpio input trait
#[macro_export]
macro_rules! impl_gpio_input_trait {
    ($p:ty) => {
        impl GpioInput for $p {}
    };
}
