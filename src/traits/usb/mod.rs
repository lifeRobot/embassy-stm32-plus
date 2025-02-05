use embassy_stm32::peripherals::{PA11, PA12};
#[cfg(USB)]
use embassy_stm32::peripherals::USB;
#[cfg(USB_OTG_FS)]
use embassy_stm32::peripherals::USB_OTG_FS;
#[cfg(USB)]
use crate::builder::usb::basis::UsbBuilder;
#[cfg(USB_OTG_FS)]
use crate::builder::usb::otg::UsbBuilder;

/// usb trait
pub trait UsbTrait {
    /// get usb builder
    fn builder(self, dp: PA12, dm: PA11) -> UsbBuilder;
}

/// usb support usb trait
#[cfg(USB)]
impl UsbTrait for USB {
    #[inline]
    fn builder(self, dp: PA12, dm: PA11) -> UsbBuilder {
        UsbBuilder::new(self, dp, dm)
    }
}

/// usb support usb trait
#[cfg(USB_OTG_FS)]
impl UsbTrait for USB_OTG_FS {
    #[inline]
    fn builder(self, dp: PA12, dm: PA11) -> UsbBuilder {
        UsbBuilder::new(self, dp, dm)
    }
}
