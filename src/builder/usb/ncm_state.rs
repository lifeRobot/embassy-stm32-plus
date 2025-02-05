use embassy_usb::class::cdc_ncm::State;

/// ncm state
pub struct NcmState<'a> {
    pub state: State<'a>,
    pub mac_address: [u8; 6],
    /// full-speed devices, `max_packet_size` has to be one of 8, 16, 32 or 64.
    pub max_packet_size: u16,
}

/// custom method
impl<'a> NcmState<'a> {
    /// full-speed devices, `max_packet_size` has to be one of 8, 16, 32 or 64.
    pub fn new(state: State<'a>, mac_address: [u8; 6], max_packet_size: u16) -> Self {
        Self { state, mac_address, max_packet_size }
    }
}

/// support default
impl<'a> Default for NcmState<'a> {
    fn default() -> Self {
        Self {
            state: State::new(),
            mac_address: [0; 6],
            max_packet_size: 64,
        }
    }
}
