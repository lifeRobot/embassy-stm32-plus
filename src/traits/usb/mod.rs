pub mod buf;
pub mod acm_state;
pub mod ncm_state;
#[cfg(any(feature = "stm32f105", feature = "stm32f107"))]
pub mod otg;
#[cfg(not(any(feature = "stm32f105", feature = "stm32f107")))]
pub mod usb_trait;
