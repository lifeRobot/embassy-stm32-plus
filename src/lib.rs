#![no_std]

// rovide the library for external use
#[cfg(feature = "embassy-stm32")]
pub use embassy_stm32;
#[cfg(feature = "embassy-time")]
pub use embassy_time;

#[cfg(feature = "stm32f1")]
pub mod stm32f1;
#[cfg(feature = "embassy-stm32")]
pub mod traits;
