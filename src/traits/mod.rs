#[cfg(all(any(ADC1, ADC2, ADC3), not(any(
    feature = "stm32c011j4",
    feature = "stm32c011j6",
    feature = "stm32c011d6",
    feature = "stm32c011f4",
    feature = "stm32c011f6",
    feature = "stm32c031f4",
    feature = "stm32c031f6",
    feature = "stm32c031g4",
    feature = "stm32c031g6",
    feature = "stm32c031k4",
    feature = "stm32c031k6",
    feature = "stm32c031c4",
    feature = "stm32c031c6",
))))]
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
#[cfg(any(I2C1, I2C2))]
pub mod i2c;
#[cfg(any(SPI1, SPI2, SPI3))]
pub mod spi;
#[cfg(any(USART1, USART2, USART3, UART4, UART5))]
pub mod uart;
pub mod uid;
#[cfg(any(USB, USB_OTG_FS))]
pub mod usb;
pub mod wdg;
