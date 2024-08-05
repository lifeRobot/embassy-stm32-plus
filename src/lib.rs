#![no_std]

// rovide the library for external use
#[cfg(feature = "embassy-stm32")]
pub use embassy_stm32;

#[cfg(feature = "stm32f1")]
pub mod stm32f1;
pub mod traits;
