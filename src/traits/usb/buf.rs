// use alloc::boxed::Box;

/// usb buffer
pub struct UsbBuf<'a> {
    pub device_descriptor: &'a mut [u8],
    pub config_descriptor: &'a mut [u8],
    pub bos_descriptor: &'a mut [u8],
    pub control_buf: &'a mut [u8],
}

/// custom method
impl<'a> UsbBuf<'a> {
    /// create usb buf
    pub fn new(device_descriptor: &'a mut [u8], config_descriptor: &'a mut [u8], bos_descriptor: &'a mut [u8], control_buf: &'a mut [u8]) -> Self {
        Self { device_descriptor, config_descriptor, bos_descriptor, control_buf }
    }
}

/*/// support default
impl Default for UsbBuf<'_> {
    /// the default method will create a 'static lifetime, please use it with caution
    fn default() -> Self {
        Self {
            device_descriptor: Box::leak(Box::new([0; 256])),
            config_descriptor: Box::leak(Box::new([0; 256])),
            bos_descriptor: Box::leak(Box::new([0; 256])),
            control_buf: Box::leak(Box::new([0; 7])),
        }
    }
}*/
