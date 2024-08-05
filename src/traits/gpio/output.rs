use embassy_stm32::gpio::{Level, Output, Pin, Speed};

/// gpio output trait
pub trait GpioOutput: Pin {
    /// default level is [Level::High], default speed is [Speed::Low]<br />
    /// these vary dpeending on the chip, ceck the reference manual or datasheet for details.<br />
    /// if it is inconsistent with the chip, please use [Self::output_with_level_speed]
    fn output(self) -> Output<'static, Self> {
        self.output_with_level_speed(Level::High, Speed::Low)
    }

    fn output_with_level_speed(self, level: Level, speed: Speed) -> Output<'static, Self> {
        Output::new(self, level, speed)
    }
}

/// impl gpio output trait
#[macro_export]
macro_rules! impl_gpio_output_trait {
    ($p:ty) => {
        impl GpioOutput for $p {}
    }
}
