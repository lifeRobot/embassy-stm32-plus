use embassy_stm32::peripherals::{PA11, PA12, USB};
use embassy_stm32::usb::Driver;
use embassy_usb::{Builder, Config, UsbDevice};
use embassy_usb::class::cdc_acm::CdcAcmClass;
use embassy_usb::class::cdc_ncm::CdcNcmClass;
use crate::builder::usb::acm_state::AcmState;
use crate::builder::usb::buf::UsbBuf;
use crate::builder::usb::ncm_state::NcmState;
use crate::irq_s::usb_can1::Irqs;

/// usb builder
pub struct UsbBuilder {
    /// usb device
    pub usb: USB,
    /// dp pin
    pub dp: PA12,
    /// dm pin
    pub dm: PA11,
}

/// custom method
impl<'a> UsbBuilder {
    /// create builder
    #[inline]
    pub fn new(usb: USB, dp: PA12, dm: PA11) -> Self {
        Self { usb, dp, dm }
    }

    /// get builder
    fn builder<const DD: usize, const CD: usize, const BD: usize, const CB: usize>
    (self, config: Config<'a>, usb_buf: &'a mut UsbBuf<DD, CD, BD, CB>) -> Builder<'a, Driver<'a, USB>> {
        let driver = Driver::new(self.usb, Irqs, self.dp, self.dm);
        Builder::new(driver, config, usb_buf.device_descriptor.as_mut(), usb_buf.config_descriptor.as_mut(), usb_buf.bos_descriptor.as_mut(), usb_buf.control_buf.as_mut())
    }

    /// build cdc acm usb
    pub fn build_cdc_acm<const DD: usize, const CD: usize, const BD: usize, const CB: usize>
    (self, config: Config<'a>, usb_buf: &'a mut UsbBuf<DD, CD, BD, CB>, state: &'a mut AcmState<'a>)
     -> (CdcAcmClass<'a, Driver<'a, USB>>, UsbDevice<'a, Driver<'a, USB>>) {
        let mut builder = self.builder(config, usb_buf);
        (CdcAcmClass::new(&mut builder, &mut state.state, state.max_packet_size), builder.build())
    }

    /// build adc ncm usb
    pub fn build_cdc_ncm<const DD: usize, const CD: usize, const BD: usize, const CB: usize>
    (self, config: Config<'a>, usb_buf: &'a mut UsbBuf<DD, CD, BD, CB>, state: &'a mut NcmState<'a>)
     -> (CdcNcmClass<'a, Driver<'a, USB>>, UsbDevice<'a, Driver<'a, USB>>) {
        let mut builder = self.builder(config, usb_buf);
        (CdcNcmClass::new(&mut builder, &mut state.state, state.mac_address, state.max_packet_size), builder.build())
    }
}
