use embassy_stm32::flash::{Blocking, Flash};
use embassy_stm32::Peripheral;
use embassy_stm32::peripherals::FLASH;

/// flash trait
pub trait FlashTrait: Peripheral<P=FLASH> + 'static {
    fn build(self) -> Flash<'static, Blocking> {
        Flash::new_blocking(self)
    }
}

/// any Flash support flash trait
impl FlashTrait for FLASH {}
