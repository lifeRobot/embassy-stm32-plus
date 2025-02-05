#[cfg(any(USART1, USART2, USART3, UART4, UART5))]
pub mod uart;
#[cfg(any(CAN, CAN1, CAN2))]
pub mod can;
#[cfg(DAC1)]
pub mod dac;
#[cfg(ETH)]
pub mod eth;
#[cfg(any(I2C1, I2C2))]
pub mod i2c;
#[cfg(any(SPI1, SPI2, SPI3))]
pub mod spi;
#[cfg(any(USB, USB_OTG_FS))]
pub mod usb;