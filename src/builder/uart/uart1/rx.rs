use embassy_stm32::mode::Async;
use embassy_stm32::Peripheral;
#[cfg(not(STM32C0))]
use embassy_stm32::peripherals::DMA1_CH5;
#[cfg(USART1_PA1)]
use embassy_stm32::peripherals::PA1;
#[cfg(PA10)]
use embassy_stm32::peripherals::PA10;
#[cfg(USART1_PA12)]
use embassy_stm32::peripherals::PA12;
#[cfg(USART1_PA14)]
use embassy_stm32::peripherals::PA14;
#[cfg(USART1_PA15)]
use embassy_stm32::peripherals::PA15;
#[cfg(USART1_PB3)]
use embassy_stm32::peripherals::PB3;
use embassy_stm32::peripherals::{PB7, USART1};
#[cfg(USART1_RX_PA8)]
use embassy_stm32::peripherals::PA8;
#[cfg(USART1_PB2)]
use embassy_stm32::peripherals::PB2;
use embassy_stm32::usart::{Config, ConfigError, RxPin, UartRx};
#[cfg(STM32C0)]
use embassy_stm32::usart::RxDma;
use crate::builder::uart::base::UartBase;
use crate::builder::uart::uart1::Irqs;

/// uart1 rx pin
pub enum Uart1Rx {
    #[cfg(USART1_PA1)]
    PA1(PA1),
    #[cfg(USART1_RX_PA8)]
    PA8(PA8),
    #[cfg(PA10)]
    PA10(PA10),
    #[cfg(USART1_PB2)]
    PB2(PB2),
    PB7(PB7),
}

/// uart1 rts pin
pub enum Uart1Rts {
    #[cfg(USART1_PA12)]
    PA12(PA12),
    #[cfg(USART1_PA14)]
    PA14(PA14),
    #[cfg(USART1_PA15)]
    PA15(PA15),
    #[cfg(USART1_PB3)]
    PB3(PB3),
}

/// uart1 rx builder
pub struct Uart1RxBuilder {
    /// uart1 base data
    pub base: UartBase<USART1>,
    /// rx pin
    pub rx: Uart1Rx,
    /// use rts
    pub rts: Option<Uart1Rts>,
}

/// uart1 rx build
macro_rules! uart1_rx_build {
    ($rx_dma:ty) => {
        /// build uart rx that supports read data
        pub fn build(self, rx_dma: $rx_dma) -> Result<UartRx<'static, Async>, ConfigError> {
            match self.rx {
                #[cfg(USART1_PA1)]
                Uart1Rx::PA1(pa1) => {Self::build_rts(pa1, rx_dma, self.base, self.rts)}
                #[cfg(USART1_RX_PA8)]
                Uart1Rx::PA8(pa8) => {Self::build_rts(pa8, rx_dma, self.base, self.rts)}
                #[cfg(PA10)]
                Uart1Rx::PA10(pa10) => { Self::build_rts(pa10, rx_dma, self.base, self.rts) }
                #[cfg(USART1_PB2)]
                Uart1Rx::PB2(pb2) => { Self::build_rts(pb2, rx_dma, self.base, self.rts) }
                Uart1Rx::PB7(pb7) => { Self::build_rts(pb7, rx_dma, self.base, self.rts) }
            }
        }

        /// build rts or default
        fn build_rts(
            rx: impl Peripheral<P=impl RxPin<USART1>> + 'static,
            rx_dma: $rx_dma,
            base: UartBase<USART1>,
            rts: Option<Uart1Rts>)
            -> Result<UartRx<'static, Async>, ConfigError> {
            let rts = crate::match_some_return!(rts,
                UartRx::new(base.uart, Irqs, rx, rx_dma, base.config.unwrap_or_default()));
            match rts {
                #[cfg(USART1_PA12)]
                Uart1Rts::PA12(pa12) => { UartRx::new_with_rts(base.uart, Irqs, rx, pa12, rx_dma, base.config.unwrap_or_default()) }
                #[cfg(USART1_PA14)]
                Uart1Rts::PA14(pa14) => { UartRx::new_with_rts(base.uart, Irqs, rx, pa14, rx_dma, base.config.unwrap_or_default()) }
                #[cfg(USART1_PA15)]
                Uart1Rts::PA15(pa15) => { UartRx::new_with_rts(base.uart, Irqs, rx, pa15, rx_dma, base.config.unwrap_or_default()) }
                #[cfg(USART1_PB3)]
                Uart1Rts::PB3(pb3) => { UartRx::new_with_rts(base.uart, Irqs, rx, pb3, rx_dma, base.config.unwrap_or_default()) }
            }
        }
    };
}

/// uart1 rx builder
impl Uart1RxBuilder {
    /// create builder
    #[inline]
    pub fn new(uart: USART1, rx: Uart1Rx) -> Self {
        Self { base: UartBase::new(uart), rx, rts: None }
    }

    /// set uart config
    #[inline]
    pub fn config(mut self, config: Config) -> Self {
        self.base.set_config(config);
        self
    }

    /// set rts
    #[inline]
    pub fn rts(mut self, rts: Uart1Rts) -> Self {
        self.rts = Some(rts);
        self
    }

    #[cfg(STM32C0)]
    uart1_rx_build!(impl Peripheral<P = impl RxDma<USART1>> + 'static);
    #[cfg(not(STM32C0))]
    uart1_rx_build!(DMA1_CH5);
}
