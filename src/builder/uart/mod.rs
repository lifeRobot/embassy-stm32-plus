pub mod base;
pub mod uart1;
pub mod uart2;
#[cfg(not(any(feature = "pin_48", feature = "pin_36")))]
pub mod uart3;