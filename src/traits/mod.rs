#[cfg(any(ADC1, ADC2, ADC3))]
pub mod adc;
#[cfg(any(CAN, CAN1, CAN2))]
pub mod can;
#[cfg(CRC)]
pub mod crc;
#[cfg(DAC1)]
pub mod dac;
#[cfg(ETH)]
pub mod eth;
pub mod flash;
pub mod gpio;
// pub mod i2c;
// pub mod spi;
#[cfg(any(USART1, USART2, USART3, UART4, UART5))]
pub mod uart;
pub mod uid;
#[cfg(feature = "embassy-usb")]
pub mod usb;
pub mod wdg;
