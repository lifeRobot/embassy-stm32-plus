use embassy_stm32::can::Can;
use embassy_stm32::peripherals::CAN;
use crate::builder::can::can1::{Can1Builder, Can1Rx, Can1Tx};

/// can1 trait
pub trait Can1Trait {
    /// build bx_can instance, more see [Can1Builder::build]
    fn build(self, tx: Can1Tx, rx: Can1Rx) -> Can<'static>;
}

impl Can1Trait for CAN {
    #[inline]
    fn build(self, tx: Can1Tx, rx: Can1Rx) -> Can<'static> {
        Can1Builder::new(self, tx, rx).build()
    }
}
