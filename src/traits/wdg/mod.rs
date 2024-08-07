use embassy_stm32::Peripheral;
use embassy_stm32::peripherals::IWDG;
use embassy_stm32::wdg::{IndependentWatchdog, Instance};

/// wdg trait
pub trait WdgTrait: Peripheral<P=Self> + Instance {
    fn build(self, timeout_us: u32) -> IndependentWatchdog<'static, Self> {
        IndependentWatchdog::new(self, timeout_us)
    }
}

/// support iwdg
impl WdgTrait for IWDG {}
