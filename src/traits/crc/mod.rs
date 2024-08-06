use embassy_stm32::crc::Crc;
use embassy_stm32::Peripheral;
use embassy_stm32::peripherals::CRC;

/// crc trait
pub trait CrcTrait: Peripheral<P=CRC> + 'static {
    fn build(self) -> Crc<'static> {
        Crc::new(self)
    }
}

/// support crc
impl CrcTrait for CRC {}
