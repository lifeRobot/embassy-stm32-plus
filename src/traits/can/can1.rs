use embassy_stm32::can::Can;
#[cfg(CAN)]
use embassy_stm32::peripherals::CAN;
#[cfg(CAN1)]
use embassy_stm32::peripherals::CAN1;
use crate::builder::can::can1::{Can1Builder, Can1Rx, Can1Tx};

/// can1 trait
pub trait Can1Trait {
    /// build bx_can instance, more see [Can1Builder::build]
    fn build(self, tx: Can1Tx, rx: Can1Rx) -> Can<'static>;
}

/// impl can 1 trait
macro_rules! impl_can1_trait {
    ($can:ty) => {
        impl Can1Trait for $can {
            #[inline]
            fn build(self, tx: Can1Tx, rx: Can1Rx) -> Can<'static> {
                Can1Builder::new(self, tx, rx).build()
            }
        }
    };
}

#[cfg(CAN)]
impl_can1_trait!(CAN);
#[cfg(CAN1)]
impl_can1_trait!(CAN1);
