#[cfg(CRC_v3)]
use embassy_stm32::crc::Config;
use embassy_stm32::crc::Crc;
use embassy_stm32::Peripheral;
use embassy_stm32::peripherals::CRC;

/// crc trait
pub trait CrcTrait: Peripheral<P=CRC> + 'static {
    #[cfg(not(CRC_v3))]
    fn build(self) -> Crc<'static> {
        Crc::new(self)
    }

    #[cfg(CRC_v3)]
    fn build(self, config: Config) -> Crc<'static> {
        Crc::new(self, config)
    }
}

/// support crc
impl CrcTrait for CRC {}
