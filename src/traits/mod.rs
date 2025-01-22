#[cfg(not(feature = "stm32l0"))]
pub mod adc;
#[cfg(not(any(
    feature = "stm32f102",
    feature = "stm32f101",
    feature = "stm32f100",
    feature = "stm32l0"
)))]
pub mod can;
#[cfg(not(feature = "stm32l0"))]
pub mod crc;
/*#[cfg(not(any(
    feature = "stm32f103vb",
    feature = "stm32f103v8",
    feature = "stm32f103tb",
    feature = "stm32f103t8",
    feature = "stm32f103t6",
    feature = "stm32f103t4",
    feature = "stm32f103rb",
    feature = "stm32f103r8",
    feature = "stm32f103r6",
    feature = "stm32f103r4",
    feature = "stm32f103cb",
    feature = "stm32f103c8",
    feature = "stm32f103c6",
    feature = "stm32f103c4",
    feature = "stm32f102",
    feature = "stm32f101vb",
    feature = "stm32f101v8",
    feature = "stm32f101rb",
    feature = "stm32f101r8",
    feature = "stm32f101r6",
    feature = "stm32f101r4",
    feature = "stm32f101cb",
    feature = "stm32f101c8",
    feature = "stm32f101c6",
    feature = "stm32f101c4",
    feature = "stm32f101tb",
    feature = "stm32f101t8",
    feature = "stm32f101t6",
    feature = "stm32f101t4",
    feature = "stm32l0"
)))]
pub mod dac;*/
#[cfg(not(feature = "stm32l0"))]
pub mod eth;
pub mod flash;
pub mod gpio;
// pub mod i2c;
// pub mod spi;
// pub mod uart;
pub mod uid;
#[cfg(feature = "embassy-usb")]
pub mod usb;
pub mod wdg;
