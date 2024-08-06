use embassy_stm32::{bind_interrupts, Peripheral, usb};
use embassy_stm32::peripherals::{PA11, PA12, USB};
use embassy_stm32::usb::{Driver, Instance};
use embassy_usb::{Builder, Config, UsbDevice};
use crate::traits::usb::buf::UsbBuf;

pub mod buf;

pub trait UsbTrait<'a, DP, DM>: Peripheral + Instance {
    fn build(self, dp: DP, dm: DM, usb_buf: UsbBuf<'a>) -> UsbDevice<'a, Driver<'a, Self>>;
}

bind_interrupts!(pub struct Irqs {
    USB_LP_CAN1_RX0 => usb::InterruptHandler<USB>;
});

impl<'a> UsbTrait<'a, PA12, PA11> for USB {
    fn build(self, dp: PA12, dm: PA11, usb_buf: UsbBuf<'a>) -> UsbDevice<'a, Driver<'a, Self>> {
        let dir = Driver::new(self, Irqs, dp, dm);
        Builder::new(dir, Config::new(0xc0de, 0xcafe), usb_buf.device_descriptor, usb_buf.config_descriptor, usb_buf.bos_descriptor, usb_buf.control_buf).build()
    }
}

pub async fn a(){
    let p = embassy_stm32::init(Default::default());
    let mut a = [0; 256];
    let mut b = [0; 256];
    let mut c = [0; 256];
    let mut d = [0; 7];
    let usb_buf = UsbBuf::new(&mut a, &mut b, &mut c, &mut d);
    let mut us = p.USB.build(p.PA12, p.PA11, usb_buf);
    us.run().await;
}
