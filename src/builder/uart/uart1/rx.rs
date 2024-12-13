use embassy_stm32::dma::NoDma;
use embassy_stm32::Peripheral;
use embassy_stm32::peripherals::{DMA1_CH5, PA10, PA12, PB7, USART1};
use embassy_stm32::usart::{Config, ConfigError, RxPin, UartRx};
use crate::builder::uart::uart1::base::Irqs;
use crate::builder::uart::uart1::Uart1Base;

/// uart1 rx pin
pub enum Uart1Rx {
    PA10(PA10),
    PB7(PB7),
}

/// uart1 rx builder
pub struct Uart1RxBuilder {
    /// uart1 base data
    pub base: Uart1Base,
    /// rx pin
    pub rx: Uart1Rx,
    /// use rts
    pub rts: Option<PA12>,
}

/// uart1 tx builder
impl Uart1RxBuilder {
    /// create builder
    #[inline]
    pub fn new(uart: USART1, rx: Uart1Rx) -> Self {
        Self { base: Uart1Base::new(uart), rx, rts: None }
    }

    /// set uart cnfig
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

    /// can not write
    #[inline]
    pub fn build_disable(self) -> Result<UartRx<'static, USART1, NoDma>, ConfigError> {
        self.build_rx(NoDma)
    }

    /// build uart rx that supports read data
    #[inline]
    pub fn build_read(self, read_dma: DMA1_CH5) -> Result<UartRx<'static, USART1, DMA1_CH5>, ConfigError> {
        self.build_rx(read_dma)
    }

    /// build by rx
    fn build_rx<RxDma>(self, rx_dma: impl Peripheral<P=RxDma> + 'static) -> Result<UartRx<'static, USART1, RxDma>, ConfigError> {
        match self.rx {
            Uart1Rx::PA10(pa10) => { Self::build_rts(pa10, rx_dma, self.base, self.rts) }
            Uart1Rx::PB7(pb7) => { Self::build_rts(pb7, rx_dma, self.base, self.rts) }
        }
    }

    /// build rts or default
    fn build_rts<RxDma>(
        rx: impl Peripheral<P=impl RxPin<USART1>> + 'static,
        rx_dma: impl Peripheral<P=RxDma> + 'static,
        base: Uart1Base,
        rts: Option<PA12>)
        -> Result<UartRx<'static, USART1, RxDma>, ConfigError> {
        let rts = crate::match_some_return!(rts,
            UartRx::new(base.uart, Irqs, rx, rx_dma, base.config.unwrap_or_default()));
        UartRx::new_with_rts(base.uart, Irqs, rx, rts, rx_dma, base.config.unwrap_or_default())
    }
}
