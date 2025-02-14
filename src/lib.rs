#![no_std]

// provide the library for external use
pub use embassy_stm32;
#[cfg(feature = "embassy-usb")]
pub use embassy_usb;
pub use cortex_m;
pub use defmt_rtt;
pub use panic_probe;
pub mod traits;
pub mod builder;
pub mod r#macro;
pub(crate) mod irq_s;