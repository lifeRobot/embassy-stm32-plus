use embassy_usb::class::cdc_acm::State;

/// acm state
pub struct AcmState<'a> {
    pub state: State<'a>,
    /// full-speed devices, `max_packet_size` has to be one of 8, 16, 32 or 64.
    pub max_packet_size: u16,
}

/// custom method
impl<'a> AcmState<'a> {
    /// full-speed devices, `max_packet_size` has to be one of 8, 16, 32 or 64.
    pub fn new(state: State<'a>, max_packet_size: u16) -> Self {
        Self { state, max_packet_size }
    }
}

/// support default
impl<'a> Default for AcmState<'a> {
    fn default() -> Self {
        Self {
            state: State::new(),
            max_packet_size: 64,
        }
    }
}
