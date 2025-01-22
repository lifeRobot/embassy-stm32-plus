use embassy_stm32::mode::Async;
use embassy_stm32::Peripheral;
use embassy_stm32::peripherals::{DMA1_CH6, PA1, PA3, USART2};
#[cfg(any(feature = "pin_100", feature = "pin_144"))]
use embassy_stm32::peripherals::{PD4, PD6};
use embassy_stm32::usart::{Config, ConfigError, RxDma, RxPin, UartRx};
use crate::builder::uart::base::UartBase;
use crate::builder::uart::uart2::Irqs;

/// uart2 rx pin
pub enum Uart2Rx {
    PA3(PA3),
    #[cfg(any(feature = "pin_100", feature = "pin_144"))]
    PD6(PD6),
}

/// uart2 rtx pin
pub enum Uart2Rts {
    PA1(PA1),
    #[cfg(any(feature = "pin_100", feature = "pin_144"))]
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

    /*/// can not read
    #[inline]
    pub fn build_disable(self) -> Result<UartRx<'static, Async>, ConfigError> {
        self.build_rx(NoDma)
    }*/

    /// build uart rx that supports read data
    #[inline]
    pub fn build_read(self, read_dma: DMA1_CH6) -> Result<UartRx<'static, Async>, ConfigError> {
        self.build_rx(read_dma)
    }

    /// build by rx
    fn build_rx(self, rx_dma: impl Peripheral<P=impl RxDma<USART2>> + 'static) -> Result<UartRx<'static, Async>, ConfigError> {
        match self.rx {
            Uart2Rx::PA3(pa3) => { Self::build_rts(pa3, rx_dma, self.base, self.rts) }
            #[cfg(any(feature = "pin_100", feature = "pin_144"))]
            Uart2Rx::PD6(pd6) => { Self::build_rts(pd6, rx_dma, self.base, self.rts) }
        }
    }

    /// build by rts
    fn build_rts(
        rx: impl Peripheral<P=impl RxPin<USART2>> + 'static,
        rx_dma: impl Peripheral<P=impl RxDma<USART2>> + 'static,
        base: UartBase<USART2>,
        rts: Option<Uart2Rts>)
        -> Result<UartRx<'static, Async>, ConfigError> {
        let rts = crate::match_some_return!(rts,
            UartRx::new(base.uart, Irqs, rx, rx_dma, base.config.unwrap_or_default()));

        match rts {
            Uart2Rts::PA1(pa1) => { UartRx::new_with_rts(base.uart, Irqs, rx, pa1, rx_dma, base.config.unwrap_or_default()) }
            #[cfg(any(feature = "pin_100", feature = "pin_144"))]
            Uart2Rts::PD4(pd4) => { UartRx::new_with_rts(base.uart, Irqs, rx, pd4, rx_dma, base.config.unwrap_or_default()) }
        }
    }
}
