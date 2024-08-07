use embassy_stm32::peripherals::UID;
use embassy_stm32::uid;

/// uid trait
pub trait UidTrait {
    #[inline]
    fn uid(&self) -> &'static [u8; 12] {
        uid::uid()
    }

    #[inline]
    fn uid_hex(&self) -> &'static str {
        uid::uid_hex()
    }

    #[inline]
    fn uid_hex_bytes(&self) -> &'static [u8; 24] {
        uid::uid_hex_bytes()
    }
}

impl UidTrait for UID {}
