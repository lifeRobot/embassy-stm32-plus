#[cfg(feature = "exti")]
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::{Input, Level, Output, Pin, Pull, Speed};
#[cfg(feature = "exti")]
use embassy_stm32::Peripheral;

/// gpio trait
pub trait GpioTrait: Pin {
    /// Create GPIO input driver, default Pull is [Pull::Up]<br />
    /// more see [Self::input_with_pull] or [Input::new]
    #[inline]
    fn input(self) -> Input<'static> {
        self.input_with_pull(Pull::Up)
    }

    /// Create GPIO input driver, more see [Input::new]
    #[inline]
    fn input_with_pull(self, pull: Pull) -> Input<'static> {
        Input::new(self, pull)
    }

    /// Create GPIO output driver, default level is [Level::High] and default speed is [Speed::Low]<br />
    /// more see [Self::output_with_level_speed] or [Output::new]
    #[inline]
    fn output(self) -> Output<'static> {
        self.output_with_level_speed(Level::High, Speed::Low)
    }

    /// Create GPIO output driver, more see [Output::new]
    #[inline]
    fn output_with_level_speed(self, level: Level, speed: Speed) -> Output<'static> {
        Output::new(self, level, speed)
    }

    /// Create an EXTI input, default pull is [Pull::Up]<br />
    /// more see [Self::exti_input_with_pull] or [ExtiInput::new]
    #[cfg(feature = "exti")]
    #[inline]
    fn exti_input(self, exti: impl Peripheral<P=Self::ExtiChannel> + 'static) -> ExtiInput<'static> {
        self.exti_input_with_pull(exti, Pull::Up)
    }

    /// Create an EXTI input, more see [ExtiInput::new]
    #[cfg(feature = "exti")]
    #[inline]
    fn exti_input_with_pull(self, exti: impl Peripheral<P=Self::ExtiChannel> + 'static, pull: Pull) -> ExtiInput<'static> {
        ExtiInput::new(self, exti, pull)
    }
}

/// any pin support gpio trait
impl<T: Pin> GpioTrait for T {}
