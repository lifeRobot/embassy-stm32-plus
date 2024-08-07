#[macro_export]
macro_rules! usb_irqs {
    () => {
        bind_interrupts!(pub struct Irqs {
            USB_LP => usb::InterruptHandler<USB>;
        });
    };
}