/// support can from 36 pin
#[macro_export]
macro_rules! impl_can_36 {
    () => {
        use embassy_stm32::{bind_interrupts, can};
        use embassy_stm32::can::Can;
        use embassy_stm32::peripherals::{CAN, PA11, PA12};
        use $crate::traits::can::CanTrait;

        bind_interrupts!(pub struct Irqs {
            USB_LP_CAN1_RX0 => can::Rx0InterruptHandler<CAN>;
            CAN1_RX1 => can::Rx1InterruptHandler<CAN>;
            CAN1_SCE => can::SceInterruptHandler<CAN>;
            USB_HP_CAN1_TX => can::TxInterruptHandler<CAN>;
        });

        $crate::impl_can_trait!(CAN,PA11,PA12);
    };
}

/// support can from 64 pin
#[macro_export]
macro_rules! impl_can_64 {
    () => {
        $crate::impl_can_36!();
        use embassy_stm32::peripherals::{PB8, PB9};

        $crate::impl_can_trait!(CAN,PA11,PB9);
        $crate::impl_can_trait!(CAN,PB8,PA12);
        $crate::impl_can_trait!(CAN,PB8,PB9);
    };
}

/// support can from 100 pin
#[macro_export]
macro_rules! impl_can_100 {
    () => {
        $crate::impl_can_64!();
        use embassy_stm32::peripherals::{PD0, PD1};

        $crate::impl_can_trait!(CAN,PA11,PD1);
        $crate::impl_can_trait!(CAN,PB8,PD1);
        $crate::impl_can_trait!(CAN,PD0,PA12);
        $crate::impl_can_trait!(CAN,PD0,PB9);
        $crate::impl_can_trait!(CAN,PD0,PD1);
    };
}
