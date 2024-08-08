use embassy_stm32::{bind_interrupts, Peripheral, usb};
use embassy_stm32::peripherals::{PA11, PA12, USB};
use embassy_stm32::usb::{Driver, Instance};
use embassy_usb::{Builder, Config, UsbDevice};
use embassy_usb::class::cdc_acm::CdcAcmClass;
use embassy_usb::class::cdc_ncm::CdcNcmClass;
use crate::traits::usb::acm_state::AcmState;
use crate::traits::usb::buf::UsbBuf;
use crate::traits::usb::ncm_state::NcmState;

pub mod buf;
pub mod acm_state;
pub mod ncm_state;

crate::usb_irqs!();

/// usb trait
pub trait UsbTrait<'a, DP, DM>: Peripheral + Instance {
    /// get usb driver
    fn driver(self, dp: DP, dm: DM) -> Driver<'a, Self>;

    /// get usb builder
    fn builder<const DD: usize, const CD: usize, const BD: usize, const CB: usize>(self, dp: DP, dm: DM, usb_buf: &'a mut UsbBuf<DD, CD, BD, CB>, config: Config<'a>) -> Builder<'a, Driver<'a, Self <>>> {
        let dri = self.driver(dp, dm);
        Builder::new(dri, config, usb_buf.device_descriptor.as_mut(), usb_buf.config_descriptor.as_mut(), usb_buf.bos_descriptor.as_mut(), usb_buf.control_buf.as_mut())
    }

    /// build usb
    fn build<const DD: usize, const CD: usize, const BD: usize, const CB: usize>(self, dp: DP, dm: DM, usb_buf: &'a mut UsbBuf<DD, CD, BD, CB>, config: Config<'a>) -> UsbDevice<'a, Driver<'a, Self>> {
        self.builder(dp, dm, usb_buf, config).build()
    }

    /// build cdc acm usb
    fn build_cdc_acm<const DD: usize, const CD: usize, const BD: usize, const CB: usize>(self, dp: DP, dm: DM, usb_buf: &'a mut UsbBuf<DD, CD, BD, CB>, state: &'a mut AcmState<'a>, config: Config<'a>) -> (CdcAcmClass<'a, Driver<'a, Self>>, UsbDevice<'a, Driver<'a, Self>>) {
        let mut builder = self.builder(dp, dm, usb_buf, config);
        (CdcAcmClass::new(&mut builder, &mut state.state, state.max_packet_size), builder.build())
    }

    /// build adc ncm usb
    fn build_cdc_ncm<const DD: usize, const CD: usize, const BD: usize, const CB: usize>(self, dp: DP, dm: DM, usb_buf: &'a mut UsbBuf<DD, CD, BD, CB>, state: &'a mut NcmState<'a>, config: Config<'a>) -> (CdcNcmClass<'a, Driver<'a, Self>>, UsbDevice<'a, Driver<'a, Self>>) {
        let mut builder = self.builder(dp, dm, usb_buf, config);
        (CdcNcmClass::new(&mut builder, &mut state.state, state.mac_address, state.max_packet_size), builder.build())
    }
}

/// support usb trait
impl<'a> UsbTrait<'a, PA12, PA11> for USB {
    fn driver(self, dp: PA12, dm: PA11) -> Driver<'a, Self> {
        Driver::new(self, Irqs, dp, dm)
    }
}
