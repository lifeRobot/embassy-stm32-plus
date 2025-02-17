use embassy_stm32::i2c::{Config, I2c, SclPin};
#[cfg(STM32C0)]
use embassy_stm32::i2c::{RxDma, TxDma};
use embassy_stm32::{bind_interrupts, i2c, Peripheral};
use embassy_stm32::mode::{Async, Blocking};
#[cfg(not(STM32C0))]
use embassy_stm32::peripherals::{DMA1_CH6, DMA1_CH7};
use embassy_stm32::peripherals::{I2C1, PB6, PB7};
#[cfg(I2C1_PA9)]
use embassy_stm32::peripherals::PA9;
#[cfg(I2C1_PA10)]
use embassy_stm32::peripherals::PA10;
#[cfg(PB8)]
use embassy_stm32::peripherals::PB8;
#[cfg(PB9)]
use embassy_stm32::peripherals::PB9;
#[cfg(I2C1_PC14)]
use embassy_stm32::peripherals::PC14;
use embassy_stm32::time::Hertz;
use crate::builder::i2c::base::I2cBase;

bind_interrupts!(struct Irqs {
    #[cfg(STM32C0)]
    I2C1 => i2c::ErrorInterruptHandler<I2C1>,i2c::EventInterruptHandler<I2C1>;
    #[cfg(not(STM32C0))]
    I2C1_ER => i2c::ErrorInterruptHandler<I2C1>;
    #[cfg(not(STM32C0))]
    I2C1_EV => i2c::EventInterruptHandler<I2C1>;
});

/// i2c1 scl pin
pub enum I2c1Scl {
    #[cfg(I2C1_PA9)]
    PA9(PA9),
    PB6(PB6),
    #[cfg(I2C1_SCL_PB7)]
    PB7(PB7),
    #[cfg(PB8)]
    PB8(PB8),
}

/// i2c1 sda pin
pub enum I2c1Sda {
    #[cfg(I2C1_PA10)]
    PA10(PA10),
    PB7(PB7),
    #[cfg(PB9)]
    PB9(PB9),
    #[cfg(I2C1_PC14)]
    PC14(PC14),
}

/// i2c1 builder
pub struct I2c1Builder {
    /// i2c base
    pub base: I2cBase<I2C1>,
    /// scl pin
    pub scl: I2c1Scl,
    /// sda pin
    pub sda: I2c1Sda,
}

/// i2c1 build method
macro_rules! i2c1_build {
    ($tx_dma:ty,$rx_dma:ty) => {
        /// Create a new I2C driver, more see [`I2c::<Async>::new`]
        pub fn build(self, tx_dma: $tx_dma, rx_dma: $rx_dma) -> I2c<'static, Async> {
            match self.scl {
                #[cfg(I2C1_PA9)]
                I2c1Scl::PA9(pa9) => { Self::build_sda(pa9, self.sda, self.base, tx_dma, rx_dma) }
                I2c1Scl::PB6(pb6) => { Self::build_sda(pb6, self.sda, self.base, tx_dma, rx_dma) }
                #[cfg(I2C1_SCL_PB7)]
                I2c1Scl::PB7(pb7) => { Self::build_sda(pb7, self.sda, self.base, tx_dma, rx_dma) }
                #[cfg(PB8)]
                I2c1Scl::PB8(pb8) => { Self::build_sda(pb8, self.sda, self.base, tx_dma, rx_dma) }
            }
        }

        /// build by sda
        pub fn build_sda(
            scl: impl Peripheral<P=impl SclPin<I2C1>> + 'static,
            sda: I2c1Sda,
            base: I2cBase<I2C1>,
            tx_dma: $tx_dma,
            rx_dma: $rx_dma) -> I2c<'static, Async> {
            match sda {
                #[cfg(I2C1_PA10)]
                I2c1Sda::PA10(pa10) => { I2c::new(base.i2c, scl, pa10, Irqs, tx_dma, rx_dma, base.freq.unwrap_or_else(|| { Hertz(1) }), base.config.unwrap_or_default()) }
                I2c1Sda::PB7(pb7) => { I2c::new(base.i2c, scl, pb7, Irqs, tx_dma, rx_dma, base.freq.unwrap_or_else(|| { Hertz(1) }), base.config.unwrap_or_default()) }
                #[cfg(PB9)]
                I2c1Sda::PB9(pb9) => { I2c::new(base.i2c, scl, pb9, Irqs, tx_dma, rx_dma, base.freq.unwrap_or_else(|| { Hertz(1) }), base.config.unwrap_or_default()) }
                #[cfg(I2C1_PC14)]
                I2c1Sda::PC14(pc14) => { I2c::new(base.i2c, scl, pc14, Irqs, tx_dma, rx_dma, base.freq.unwrap_or_else(|| { Hertz(1) }), base.config.unwrap_or_default()) }
            }
        }
    };
}

/// custom method
impl I2c1Builder {
    /// create builder
    #[inline]
    pub fn new(i2c: I2C1, scl: I2c1Scl, sda: I2c1Sda) -> Self {
        Self { base: I2cBase::new(i2c), scl, sda }
    }

    /// set i2c freq hertz
    #[inline]
    pub fn freq(mut self, freq: Hertz) -> Self {
        self.base.set_freq(freq);
        self
    }

    /// set i2c config
    #[inline]
    pub fn config(mut self, config: Config) -> Self {
        self.base.set_config(config);
        self
    }

    #[cfg(STM32C0)]
    i2c1_build!(impl Peripheral<P = impl TxDma<I2C1>> + 'static,impl Peripheral<P = impl RxDma<I2C1>> + 'static);
    #[cfg(not(STM32C0))]
    i2c1_build!(DMA1_CH6,DMA1_CH7);

    /// Create a new I2C driver, more see [`I2c::<Blocking>::new_blocking`]
    pub fn build_blocking(self) -> I2c<'static, Blocking> {
        match self.scl {
            #[cfg(I2C1_PA9)]
            I2c1Scl::PA9(pa9) => { Self::build_blocking_sda(pa9, self.sda, self.base) }
            I2c1Scl::PB6(pb6) => { Self::build_blocking_sda(pb6, self.sda, self.base) }
            #[cfg(I2C1_SCL_PB7)]
            I2c1Scl::PB7(pb7) => { Self::build_blocking_sda(pb7, self.sda, self.base) }
            #[cfg(PB8)]
            I2c1Scl::PB8(pb8) => { Self::build_blocking_sda(pb8, self.sda, self.base) }
        }
    }

    /// build by sda
    pub fn build_blocking_sda(
        scl: impl Peripheral<P=impl SclPin<I2C1>> + 'static,
        sda: I2c1Sda,
        base: I2cBase<I2C1>) -> I2c<'static, Blocking> {
        match sda {
            #[cfg(I2C1_PA10)]
            I2c1Sda::PA10(pa10) => { I2c::new_blocking(base.i2c, scl, pa10, base.freq.unwrap_or_else(|| { Hertz(1) }), base.config.unwrap_or_default()) }
            I2c1Sda::PB7(pb7) => { I2c::new_blocking(base.i2c, scl, pb7, base.freq.unwrap_or_else(|| { Hertz(1) }), base.config.unwrap_or_default()) }
            #[cfg(PB9)]
            I2c1Sda::PB9(pb9) => { I2c::new_blocking(base.i2c, scl, pb9, base.freq.unwrap_or_else(|| { Hertz(1) }), base.config.unwrap_or_default()) }
            #[cfg(I2C1_PC14)]
            I2c1Sda::PC14(pc14) => { I2c::new_blocking(base.i2c, scl, pc14, base.freq.unwrap_or_else(|| { Hertz(1) }), base.config.unwrap_or_default()) }
        }
    }
}
