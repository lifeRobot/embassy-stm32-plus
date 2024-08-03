#![no_std]

// rovide the library for external use
pub use embassy_stm32;

#[cfg(feature = "stm32f1")]
pub mod stm32f1;