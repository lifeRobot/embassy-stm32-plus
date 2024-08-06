use embassy_stm32::can::{Can, Instance};
use embassy_stm32::Peripheral;

/// can trait
pub trait CanTrait<Rx, Tx>: Peripheral + Instance {
    fn build(self, rx: Rx, tx: Tx) -> Can<'static, Self>;
}

/// impl can trait
#[macro_export]
macro_rules! impl_can_trait {
    ($can:ty,$rx:ty,$tx:ty) => {
        impl CanTrait<$rx,$tx> for $can {
            fn build(self, rx: $rx, tx: $tx) -> Can<'static, Self> {
                Can::new(self,rx,tx,Irqs)
            }
        }
    };
}
