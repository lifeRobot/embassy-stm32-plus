use embassy_stm32::{bind_interrupts, Peripheral, usb_otg};
use embassy_stm32::peripherals::{PA11, PA12, USB_OTG_FS};
pub use embassy_stm32::usb_otg::{Config as OtgConfig, Driver, Instance};
use embassy_usb::{Builder, Config, UsbDevice};
use embassy_usb::class::cdc_acm::CdcAcmClass;
use embassy_usb::class::cdc_ncm::CdcNcmClass;
use crate::traits::usb::acm_state::AcmState;
use crate::traits::usb::buf::UsbBuf;
use crate::traits::usb::ncm_state::NcmState;

bind_interrupts!(pub struct Irqs {
    OTG_FS => usb_otg::InterruptHandler<USB_OTG_FS>;
});

/// usb otg trait
pub trait UsbOtgTrait<'a, DP, DM>: Peripheral + Instance {
    /// get usb driver with config<br />
    /// * `ep_out_buffer` - An internal buffer used to temporarily store recevied packets.
    /// Must be large enough to fit all OUT endpoint max packet sizes.
    /// Endpoint allocation will fail if it is too small.
    fn driver_with_config(self, dp: DP, dm: DM, ep_out_buffer: &'a mut [u8], otg_config: OtgConfig) -> Driver<'a, Self>;

    /// get usb driver <br />
    /// * `ep_out_buffer` - An internal buffer used to temporarily store recevied packets.
    /// Must be large enough to fit all OUT endpoint max packet sizes.
    /// Endpoint allocation will fail if it is too small.
    fn driver(self, dp: DP, dm: DM, ep_out_buffer: &'a mut [u8]) -> Driver<'a, Self> {
        let mut otg_config = OtgConfig::default();
        otg_config.vbus_detection = false;
        self.driver_with_config(dp, dm, ep_out_buffer, otg_config)
    }

    /// get usb builder with otg config
    fn builder_with_config<const DD: usize, const CD: usize, const BD: usize, const CB: usize>(self, dp: DP, dm: DM, ep_out_buffer: &'a mut [u8], usb_buf: &'a mut UsbBuf<DD, CD, BD, CB>, otg_config: OtgConfig, config: Config<'a>) -> Builder<'a, Driver<'a, Self>> {
        let dri = self.driver_with_config(dp, dm, ep_out_buffer, otg_config);
        Builder::new(dri, config, usb_buf.device_descriptor.as_mut(), usb_buf.config_descriptor.as_mut(), usb_buf.bos_descriptor.as_mut(), usb_buf.control_buf.as_mut())
    }

    /// get usb builder
    fn builder<const DD: usize, const CD: usize, const BD: usize, const CB: usize>(self, dp: DP, dm: DM, ep_out_buffer: &'a mut [u8], usb_buf: &'a mut UsbBuf<DD, CD, BD, CB>, config: Config<'a>) -> Builder<'a, Driver<'a, Self>> {
        let mut otg_config = OtgConfig::default();
        otg_config.vbus_detection = false;
        self.builder_with_config(dp, dm, ep_out_buffer, usb_buf, otg_config, config)
    }

    /// build cdc acm usb with otg config
    fn build_cdc_acm_with_config<const DD: usize, const CD: usize, const BD: usize, const CB: usize>(self, dp: DP, dm: DM, ep_out_buffer: &'a mut [u8], usb_buf: &'a mut UsbBuf<DD, CD, BD, CB>, state: &'a mut AcmState<'a>, otg_config: OtgConfig, config: Config<'a>) -> (CdcAcmClass<'a, Driver<'a, Self>>, UsbDevice<'a, Driver<'a, Self>>) {
        let mut builder = self.builder_with_config(dp, dm, ep_out_buffer, usb_buf, otg_config, config);
        (CdcAcmClass::new(&mut builder, &mut state.state, state.max_packet_size), builder.build())
    }

    /// build cdc acm usb
    fn build_cdc_acm<const DD: usize, const CD: usize, const BD: usize, const CB: usize>(self, dp: DP, dm: DM, ep_out_buffer: &'a mut [u8], usb_buf: &'a mut UsbBuf<DD, CD, BD, CB>, state: &'a mut AcmState<'a>, config: Config<'a>) -> (CdcAcmClass<'a, Driver<'a, Self>>, UsbDevice<'a, Driver<'a, Self>>) {
        let mut otg_config = OtgConfig::default();
        otg_config.vbus_detection = false;
        self.build_cdc_acm_with_config(dp, dm, ep_out_buffer, usb_buf, state, otg_config, config)
    }

    /// build adc ncm usb with otg config
    fn build_cdc_ncm_with_config<const DD: usize, const CD: usize, const BD: usize, const CB: usize>(self, dp: DP, dm: DM, ep_out_buffer: &'a mut [u8], usb_buf: &'a mut UsbBuf<DD, CD, BD, CB>, state: &'a mut NcmState<'a>, otg_config: OtgConfig, config: Config<'a>) -> (CdcNcmClass<'a, Driver<'a, Self>>, UsbDevice<'a, Driver<'a, Self>>) {
        let mut builder = self.builder_with_config(dp, dm, ep_out_buffer, usb_buf, otg_config, config);
        (CdcNcmClass::new(&mut builder, &mut state.state, state.mac_address, state.max_packet_size), builder.build())
    }

    /// build adc ncm usb
    fn build_cdc_ncm<const DD: usize, const CD: usize, const BD: usize, const CB: usize>(self, dp: DP, dm: DM, ep_out_buffer: &'a mut [u8], usb_buf: &'a mut UsbBuf<DD, CD, BD, CB>, state: &'a mut NcmState<'a>, config: Config<'a>) -> (CdcNcmClass<'a, Driver<'a, Self>>, UsbDevice<'a, Driver<'a, Self>>) {
        let mut otg_config = OtgConfig::default();
        otg_config.vbus_detection = false;
        self.build_cdc_ncm_with_config(dp, dm, ep_out_buffer, usb_buf, state, otg_config, config)
    }
}

/// support usb otg trait
impl<'a> UsbOtgTrait<'a, PA12, PA11> for USB_OTG_FS {
    fn driver_with_config(self, dp: PA12, dm: PA11, ep_out_buffer: &'a mut [u8], config: OtgConfig) -> Driver<'a, Self> {
        Driver::new_fs(self, Irqs, dp, dm, ep_out_buffer, config)
    }
}
