use embassy_stm32::mode::Async;
use embassy_stm32::Peripheral;
#[cfg(not(STM32C0))]
use embassy_stm32::peripherals::DMA1_CH5;
#[cfg(STM32C0)]
use embassy_stm32::peripherals::DMA1_CH2;
use embassy_stm32::peripherals::{PA10, PA12, PB7, USART1};
use embassy_stm32::usart::{Config, ConfigError, RxPin, UartRx};
use crate::builder::uart::base::UartBase;
use crate::builder::uart::uart1::Irqs;

/// uart1 rx pin
pub enum Uart1Rx {
    PA10(PA10),
    PB7(PB7),
}

/// uart1 rx builder
pub struct Uart1RxBuilder {
    /// uart1 base data
    pub base: UartBase<USART1>,
    /// rx pin
    pub rx: Uart1Rx,
    /// use rts
    pub rts: Option<PA12>,
}

/// uart1 rx build
macro_rules! uart1_rx_build {
    ($rx_dma:ty) => {
        /// build uart rx that supports read data
        pub fn build(self, rx_dma: $rx_dma) -> Result<UartRx<'static, Async>, ConfigError> {
            match self.rx {
                Uart1Rx::PA10(pa10) => { Self::build_rts(pa10, rx_dma, self.base, self.rts) }
                Uart1Rx::PB7(pb7) => { Self::build_rts(pb7, rx_dma, self.base, self.rts) }
            }
        }

        /// build rts or default
        fn build_rts(
            rx: impl Peripheral<P=impl RxPin<USART1>> + 'static,
            rx_dma: $rx_dma,
            base: UartBase<USART1>,
            rts: Option<PA12>)
            -> Result<UartRx<'static, Async>, ConfigError> {
            let rts = crate::match_some_return!(rts,
                UartRx::new(base.uart, Irqs, rx, rx_dma, base.config.unwrap_or_default()));
            UartRx::new_with_rts(base.uart, Irqs, rx, rts, rx_dma, base.config.unwrap_or_default())
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
    pub fn rts(mut self, rts: PA12) -> Self {
        self.rts = Some(rts);
        self
    }

    #[cfg(STM32C0)]
    uart1_rx_build!(DMA1_CH2);
    #[cfg(not(STM32C0))]
    uart1_rx_build!(DMA1_CH5);
    /*/// build uart rx that supports read data
    pub fn build(self, rx_dma: DMA1_CH5) -> Result<UartRx<'static, Async>, ConfigError> {
        match self.rx {
            Uart1Rx::PA10(pa10) => { Self::build_rts(pa10, rx_dma, self.base, self.rts) }
            Uart1Rx::PB7(pb7) => { Self::build_rts(pb7, rx_dma, self.base, self.rts) }
        }
    }

    /// build rts or default
    fn build_rts(
        rx: impl Peripheral<P=impl RxPin<USART1>> + 'static,
        rx_dma: DMA1_CH5,
        base: UartBase<USART1>,
        rts: Option<PA12>)
        -> Result<UartRx<'static, Async>, ConfigError> {
        let rts = crate::match_some_return!(rts,
            UartRx::new(base.uart, Irqs, rx, rx_dma, base.config.unwrap_or_default()));
        UartRx::new_with_rts(base.uart, Irqs, rx, rts, rx_dma, base.config.unwrap_or_default())
    }*/
}
