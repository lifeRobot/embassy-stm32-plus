use embassy_stm32::bind_interrupts;
#[cfg(any(CAN, CAN1))]
use embassy_stm32::can;
#[cfg(USB)]
use embassy_stm32::usb;
#[cfg(CAN)]
use embassy_stm32::peripherals::CAN;
#[cfg(CAN1)]
use embassy_stm32::peripherals::CAN1;
#[cfg(USB)]
use embassy_stm32::peripherals::USB;

#[cfg(CAN1)]
bind_interrupts!(pub struct Irqs {
    CAN1_RX0 => can::Rx0InterruptHandler<CAN1>;
    CAN1_RX1 => can::Rx1InterruptHandler<CAN1>;
    CAN1_SCE => can::SceInterruptHandler<CAN1>;
    CAN1_TX => can::TxInterruptHandler<CAN1>;
});

#[cfg(not(CAN1))]
bind_interrupts!(pub struct Irqs {
    #[cfg(all(CAN,USB,STM32F103))]
    USB_LP_CAN1_RX0 => can::Rx0InterruptHandler<CAN>,usb::InterruptHandler<USB>;
    #[cfg(all(not(CAN),USB,STM32F103))]
    USB_LP_CAN1_RX0 => usb::InterruptHandler<USB>;
    #[cfg(all(CAN,not(USB)))]
    USB_LP_CAN1_RX0 => can::Rx0InterruptHandler<CAN>;

    #[cfg(CAN)]
    CAN1_RX1 => can::Rx1InterruptHandler<CAN>;
    #[cfg(CAN)]
    CAN1_SCE => can::SceInterruptHandler<CAN>;
    #[cfg(all(USB,STM32F102))]
    USB_LP => usb::InterruptHandler<USB>;
    #[cfg(CAN)]
    USB_HP_CAN1_TX => can::TxInterruptHandler<CAN>;
});
