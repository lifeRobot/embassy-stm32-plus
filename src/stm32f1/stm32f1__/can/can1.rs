/// support can1 from 64 pin
#[macro_export]
macro_rules! impl_can1_64 {
    () => {
        use embassy_stm32::{bind_interrupts, can};
        use embassy_stm32::can::Can;
        use embassy_stm32::peripherals::{CAN1, PA11, PA12, PB8, PB9};
        use $crate::traits::can::CanTrait;

        bind_interrupts!(pub struct Irqs {
            CAN1_RX0 => can::Rx0InterruptHandler<CAN1>;
            CAN1_RX1 => can::Rx1InterruptHandler<CAN1>;
            CAN1_SCE => can::SceInterruptHandler<CAN1>;
            CAN1_TX => can::TxInterruptHandler<CAN1>;
        });

        $crate::impl_can_trait!(CAN1,PA11,PA12);
        $crate::impl_can_trait!(CAN1,PA11,PB9);
        $crate::impl_can_trait!(CAN1,PB8,PA12);
        $crate::impl_can_trait!(CAN1,PB8,PB9);
    };
}

/// suport can1 from 64 pin
#[macro_export]
macro_rules! mod_can1_64 {
    () => {
        pub mod can1 {
            $crate::impl_can1_64!();
        }
    };
}

/// support can1 from 100 pin
#[macro_export]
macro_rules! mod_can1_100 {
    () => {
        pub mod can1 {
            $crate::impl_can1_64!();

            use embassy_stm32::peripherals::{PD0, PD1};

            $crate::impl_can_trait!(CAN1,PA11,PD1);
            $crate::impl_can_trait!(CAN1,PB8,PD1);
            $crate::impl_can_trait!(CAN1,PD0,PA12);
            $crate::impl_can_trait!(CAN1,PD0,PB9);
            $crate::impl_can_trait!(CAN1,PD0,PD1);
        }
    };
}
