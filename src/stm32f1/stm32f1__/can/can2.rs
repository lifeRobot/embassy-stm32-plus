#[macro_export]
macro_rules! mod_can2 {
    () => {
        pub mod can2{
            use embassy_stm32::{bind_interrupts, can};
            use embassy_stm32::can::Can;
            use embassy_stm32::peripherals::{CAN2, PB12, PB13, PB5, PB6};
            use $crate::traits::can::CanTrait;

            bind_interrupts!(pub struct Irqs {
                CAN2_RX0 => can::Rx0InterruptHandler<CAN2>;
                CAN2_RX1 => can::Rx1InterruptHandler<CAN2>;
                CAN2_SCE => can::SceInterruptHandler<CAN2>;
                CAN2_TX => can::TxInterruptHandler<CAN2>;
            });

            $crate::impl_can_trait!(CAN2,PB5,PB6);
            $crate::impl_can_trait!(CAN2,PB5,PB13);
            $crate::impl_can_trait!(CAN2,PB12,PB6);
            $crate::impl_can_trait!(CAN2,PB12,PB13);
        }
    };
}