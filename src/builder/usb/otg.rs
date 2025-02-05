use embassy_stm32::{bind_interrupts, usb};
use embassy_stm32::peripherals::{PA11, PA12, USB_OTG_FS};
use embassy_stm32::usb::{Config, Driver};
use embassy_usb::{Builder, UsbDevice};
use embassy_usb::class::cdc_acm::CdcAcmClass;
use embassy_usb::class::cdc_ncm::CdcNcmClass;
use crate::builder::usb::acm_state::AcmState;
use crate::builder::usb::buf::UsbBuf;
use crate::builder::usb::ncm_state::NcmState;

bind_interrupts!(struct Irqs {
    OTG_FS => usb::InterruptHandler<USB_OTG_FS>;
});

/// usb builder
pub struct UsbBuilder {
    /// usb device
    pub usb: USB_OTG_FS,
    /// dp pin
    pub dp: PA12,
    /// dm pin
    pub dm: PA11,
    /// otg usb config
    pub config: Option<Config>,
}

/// custom method
impl UsbBuilder {
    /// create builder
    #[inline]
    pub fn new(usb: USB_OTG_FS, dp: PA12, dm: PA11) -> Self {
        Self { usb, dp, dm, config: None }
    }

    /// set otg usb config
    #[inline]
    pub fn config(mut self, config: Config) -> Self {
        self.config = Some(config);
        self
    }

    /// get builder
    fn builder<'a, const DD: usize, const CD: usize, const BD: usize, const CB: usize>
    (self, ep_out_buffer: &'a mut [u8], usb_buf: &'a mut UsbBuf<DD, CD, BD, CB>, config: embassy_usb::Config<'a>)
     -> Builder<'a, Driver<'a, USB_OTG_FS>> {
        let driver = Driver::new_fs(self.usb, Irqs, self.dp, self.dm, ep_out_buffer, self.config.unwrap_or_default());
        Builder::new(driver, config, usb_buf.device_descriptor.as_mut(), usb_buf.config_descriptor.as_mut(), usb_buf.bos_descriptor.as_mut(), usb_buf.control_buf.as_mut())
    }

    /// build cdc acm usb
    pub fn build_cdc_acm<'a, const DD: usize, const CD: usize, const BD: usize, const CB: usize>
    (self, config: embassy_usb::Config<'a>, ep_out_buffer: &'a mut [u8], usb_buf: &'a mut UsbBuf<DD, CD, BD, CB>, state: &'a mut AcmState<'a>)
     -> (CdcAcmClass<'a, Driver<'a, USB_OTG_FS>>, UsbDevice<'a, Driver<'a, USB_OTG_FS>>) {
        let mut builder = self.builder(ep_out_buffer, usb_buf, config);
        (CdcAcmClass::new(&mut builder, &mut state.state, state.max_packet_size), builder.build())
    }

    /// build adc ncm usb
    pub fn build_cdc_ncm<'a, const DD: usize, const CD: usize, const BD: usize, const CB: usize>
    (self, config: embassy_usb::Config<'a>, ep_out_buffer: &'a mut [u8], usb_buf: &'a mut UsbBuf<DD, CD, BD, CB>, state: &'a mut NcmState<'a>)
     -> (CdcNcmClass<'a, Driver<'a, USB_OTG_FS>>, UsbDevice<'a, Driver<'a, USB_OTG_FS>>) {
        let mut builder = self.builder(ep_out_buffer, usb_buf, config);
        (CdcNcmClass::new(&mut builder, &mut state.state, state.mac_address, state.max_packet_size), builder.build())
    }
}
