#![no_std]

// provide the library for external use
#[cfg(feature = "embassy-stm32")]
pub use embassy_stm32;
#[cfg(feature = "embassy-usb")]
pub use embassy_usb;

#[cfg(feature = "stm32f1")]
pub mod stm32f1;
#[cfg(feature = "embassy-stm32")]
pub mod traits;
#[cfg(feature = "embassy-stm32")]
pub mod builder;
pub mod r#macro;