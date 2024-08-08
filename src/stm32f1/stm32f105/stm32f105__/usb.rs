#[macro_export]
macro_rules! usb_irqs {
    () => {
        bind_interrupts!(pub struct Irqs {
            USB_LP_CAN1_RX0 => usb::InterruptHandler<USB>;
        });
    };
}