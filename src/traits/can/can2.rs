use embassy_stm32::can::Can;
use embassy_stm32::peripherals::CAN2;
use crate::builder::can::can2::{Can2Builder, Can2Rx, Can2Tx};

/// can2 trait
pub trait Can2Trait {
    /// build bx_can instance, more see [Can2Builder::build]
    fn build(self, tx: Can2Tx, rx: Can2Rx) -> Can<'static>;
}

/// CAN2 support can2 trait
impl Can2Trait for CAN2 {
    #[inline]
    fn build(self, tx: Can2Tx, rx: Can2Rx) -> Can<'static> {
        Can2Builder::new(self, tx, rx).build()
    }
}
