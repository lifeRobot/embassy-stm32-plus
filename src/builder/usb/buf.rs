/// usb buffer
pub struct UsbBuf<const DD: usize, const CD: usize, const BD: usize, const CB: usize> {
    pub device_descriptor: [u8; DD],
    pub config_descriptor: [u8; CD],
    pub bos_descriptor: [u8; BD],
    pub control_buf: [u8; CB],
}

/// custom method
impl<const DD: usize, const CD: usize, const BD: usize, const CB: usize> UsbBuf<DD, CD, BD, CB> {
    /// create usb buf
    pub fn new(device_descriptor: [u8; DD], config_descriptor: [u8; CD], bos_descriptor: [u8; BD], control_buf: [u8; CB]) -> Self {
        Self { device_descriptor, config_descriptor, bos_descriptor, control_buf }
    }
}

/// support default
impl Default for UsbBuf<256, 256, 256, 7> {
    fn default() -> Self {
        Self {
            device_descriptor: [0; 256],
            config_descriptor: [0; 256],
            bos_descriptor: [0; 256],
            control_buf: [0; 7],
        }
    }
}
