use embassy_stm32::mode::Async;
use embassy_stm32::Peripheral;
use embassy_stm32::peripherals::{DMA1_CH3, PB11, PB14, USART3};
#[cfg(PC11)]
use embassy_stm32::peripherals::PC11;
#[cfg(PD12)]
use embassy_stm32::peripherals::PD12;
#[cfg(PD9)]
use embassy_stm32::peripherals::PD9;
use embassy_stm32::usart::{Config, ConfigError, RxPin, UartRx};
use crate::builder::uart::base::UartBase;
use crate::builder::uart::uart3::Irqs;

/// uart3 rx pin
pub enum Uart3Rx {
    PB11(PB11),
    #[cfg(PC11)]
    PC11(PC11),
    #[cfg(PD9)]
    PD9(PD9),
}

/// uart3 rtx pin
pub enum Uart3Rts {
    PB14(PB14),
    #[cfg(PD12)]
    PD12(PD12),
}

/// uart3 rx builder
pub struct Uart3RxBuilder {
    /// uart3 base device
    pub base: UartBase<USART3>,
    /// rx pin
    pub rx: Uart3Rx,
    /// use rts
    pub rts: Option<Uart3Rts>,
}

/// custom method
impl Uart3RxBuilder {
    /// create builder
    #[inline]
    pub fn new(uart: USART3, rx: Uart3Rx) -> Self {
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
    pub fn rts(mut self, rts: Uart3Rts) -> Self {
        self.rts = Some(rts);
        self
    }

    /// build uart rx that supports read data
    pub fn build(self, rx_dma: DMA1_CH3) -> Result<UartRx<'static, Async>, ConfigError> {
        match self.rx {
            Uart3Rx::PB11(pb11) => { Self::build_rts(pb11, rx_dma, self.base, self.rts) }
            #[cfg(PC11)]
            Uart3Rx::PC11(pc11) => { Self::build_rts(pc11, rx_dma, self.base, self.rts) }
            #[cfg(PD9)]
            Uart3Rx::PD9(pd9) => { Self::build_rts(pd9, rx_dma, self.base, self.rts) }
        }
    }

    /// build by rts
    fn build_rts(
        rx: impl Peripheral<P=impl RxPin<USART3>> + 'static,
        rx_dma: DMA1_CH3,
        base: UartBase<USART3>,
        rts: Option<Uart3Rts>)
        -> Result<UartRx<'static, Async>, ConfigError> {
        let rts = crate::match_some_return!(rts,
            UartRx::new(base.uart, Irqs, rx, rx_dma, base.config.unwrap_or_default()));

        match rts {
            Uart3Rts::PB14(pb14) => { UartRx::new_with_rts(base.uart, Irqs, rx, pb14, rx_dma, base.config.unwrap_or_default()) }
            #[cfg(PD12)]
            Uart3Rts::PD12(pd12) => { UartRx::new_with_rts(base.uart, Irqs, rx, pd12, rx_dma, base.config.unwrap_or_default()) }
        }
    }
}
