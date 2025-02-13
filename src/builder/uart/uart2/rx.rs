use embassy_stm32::mode::Async;
use embassy_stm32::Peripheral;
#[cfg(not(STM32C0))]
use embassy_stm32::peripherals::DMA1_CH6;
use embassy_stm32::peripherals::{PA1, USART2};
#[cfg(USART2_PA3)]
use embassy_stm32::peripherals::PA3;
#[cfg(USART2_PA5)]
use embassy_stm32::peripherals::PA5;
#[cfg(USART2_PA13)]
use embassy_stm32::peripherals::PA13;
#[cfg(USART2_PA14)]
use embassy_stm32::peripherals::PA14;
#[cfg(USART2_PA15)]
use embassy_stm32::peripherals::PA15;
#[cfg(USART2_PB9)]
use embassy_stm32::peripherals::PB9;
#[cfg(USART2_PC14)]
use embassy_stm32::peripherals::PC14;
#[cfg(USART2_PD4)]
use embassy_stm32::peripherals::PD4;
#[cfg(USART2_PD6)]
use embassy_stm32::peripherals::PD6;
use embassy_stm32::usart::{Config, ConfigError, RxPin, UartRx};
#[cfg(STM32C0)]
use embassy_stm32::usart::RxDma;
use crate::builder::uart::base::UartBase;
use crate::builder::uart::uart2::Irqs;

/// uart2 rx pin
pub enum Uart2Rx {
    #[cfg(USART2_PA3)]
    PA3(PA3),
    #[cfg(USART2_PA5)]
    PA5(PA5),
    #[cfg(USART2_PA13)]
    PA13(PA13),
    #[cfg(USART2_PA14)]
    PA14(PA14),
    #[cfg(USART2_PA15)]
    PA15(PA15),
    #[cfg(USART2_PD6)]
    PD6(PD6),
}

/// uart2 rtx pin
pub enum Uart2Rts {
    PA1(PA1),
    #[cfg(USART2_PB9)]
    PB9(PB9),
    #[cfg(USART2_PC14)]
    PC14(PC14),
    #[cfg(USART2_PD4)]
    PD4(PD4),
}

/// uart2 rx builder
pub struct Uart2RxBuilder {
    /// uart2 base device
    pub base: UartBase<USART2>,
    /// rx pin
    pub rx: Uart2Rx,
    /// use rts
    pub rts: Option<Uart2Rts>,
}

/// uart2 rx build
macro_rules! uart2_rx_build {
    ($rx_dma:ty) => {
        /// build uart rx that supports read data
        pub fn build(self, rx_dma: $rx_dma) -> Result<UartRx<'static, Async>, ConfigError> {
            match self.rx {
                #[cfg(USART2_PA3)]
                Uart2Rx::PA3(pa3) => { Self::build_rts(pa3, rx_dma, self.base, self.rts) }
                #[cfg(USART2_PA5)]
                Uart2Rx::PA5(pa5) => { Self::build_rts(pa5, rx_dma, self.base, self.rts) }
                #[cfg(USART2_PA13)]
                Uart2Rx::PA13(pa13) => { Self::build_rts(pa13, rx_dma, self.base, self.rts) }
                #[cfg(USART2_PA14)]
                Uart2Rx::PA14(pa14) => { Self::build_rts(pa14, rx_dma, self.base, self.rts) }
                #[cfg(USART2_PA15)]
                Uart2Rx::PA15(pa15) => { Self::build_rts(pa15, rx_dma, self.base, self.rts) }
                #[cfg(USART2_PD6)]
                Uart2Rx::PD6(pd6) => { Self::build_rts(pd6, rx_dma, self.base, self.rts) }
            }
        }

        /// build by rts
        fn build_rts(
            rx: impl Peripheral<P=impl RxPin<USART2>> + 'static,
            rx_dma: $rx_dma,
            base: UartBase<USART2>,
            rts: Option<Uart2Rts>)
            -> Result<UartRx<'static, Async>, ConfigError> {
            let rts = crate::match_some_return!(rts,
                UartRx::new(base.uart, Irqs, rx, rx_dma, base.config.unwrap_or_default()));

            match rts {
                Uart2Rts::PA1(pa1) => { UartRx::new_with_rts(base.uart, Irqs, rx, pa1, rx_dma, base.config.unwrap_or_default()) }
                #[cfg(USART2_PB9)]
                Uart2Rts::PB9(pb9) => { UartRx::new_with_rts(base.uart, Irqs, rx, pb9, rx_dma, base.config.unwrap_or_default()) }
                #[cfg(USART2_PC14)]
                Uart2Rts::PC14(pb14) => { UartRx::new_with_rts(base.uart, Irqs, rx, pb14, rx_dma, base.config.unwrap_or_default()) }
                #[cfg(USART2_PD4)]
                Uart2Rts::PD4(pd4) => { UartRx::new_with_rts(base.uart, Irqs, rx, pd4, rx_dma, base.config.unwrap_or_default()) }
            }
        }
    };
}

/// custom method
impl Uart2RxBuilder {
    /// create builder
    #[inline]
    pub fn new(uart: USART2, rx: Uart2Rx) -> Self {
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
    pub fn rts(mut self, rts: Uart2Rts) -> Self {
        self.rts = Some(rts);
        self
    }

    #[cfg(STM32C0)]
    uart2_rx_build!(impl Peripheral<P = impl RxDma<USART2>> + 'static);
    #[cfg(not(STM32C0))]
    uart2_rx_build!(DMA1_CH6);
}
