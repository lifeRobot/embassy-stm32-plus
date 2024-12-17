use embassy_stm32::dma::NoDma;
use embassy_stm32::Peripheral;
use embassy_stm32::peripherals::{DMA1_CH7, PA0, PA2, USART2};
#[cfg(any(feature = "pin_100", feature = "pin_144"))]
use embassy_stm32::peripherals::{PD3, PD5};
use embassy_stm32::usart::{Config, ConfigError, TxPin, UartTx};
use crate::builder::uart::base::UartBase;

/// uart2 tx pin
pub enum Uart2Tx {
    PA2(PA2),
    #[cfg(any(feature = "pin_100", feature = "pin_144"))]
    PD5(PD5),
}

/// uart2 cts pin
pub enum Uart2Cts {
    PA0(PA0),
    #[cfg(any(feature = "pin_100", feature = "pin_144"))]
    PD3(PD3),
}

/// uart2 tx builder
pub struct Uart2TxBuilder {
    /// uart2 base data
    pub base: UartBase<USART2>,
    /// tx pin
    pub tx: Uart2Tx,
    /// use cts
    pub cts: Option<Uart2Cts>,
}

/// custom method
impl Uart2TxBuilder {
    /// create builder
    #[inline]
    pub fn new(uart: USART2, tx: Uart2Tx) -> Self {
        Self { base: UartBase::new(uart), tx, cts: None }
    }

    /// set uart config
    #[inline]
    pub fn config(mut self, config: Config) -> Self {
        self.base.set_config(config);
        self
    }

    /// set cts pin
    #[inline]
    pub fn cts(mut self, cts: Uart2Cts) -> Self {
        self.cts = Some(cts);
        self
    }

    /// can not write
    #[inline]
    pub fn build_disable(self) -> Result<UartTx<'static, USART2, NoDma>, ConfigError> {
        self.build_tx(NoDma)
    }

    /// build uart tx that supports write data
    #[inline]
    pub fn build_write(self, write_dma: DMA1_CH7) -> Result<UartTx<'static, USART2, DMA1_CH7>, ConfigError> {
        self.build_tx(write_dma)
    }

    /// build by tx
    fn build_tx<TxDma>(self, tx_dma: impl Peripheral<P=TxDma> + 'static) -> Result<UartTx<'static, USART2, TxDma>, ConfigError> {
        match self.tx {
            Uart2Tx::PA2(pa2) => { Self::build_cts(pa2, tx_dma, self.base, self.cts) }
            #[cfg(any(feature = "pin_100", feature = "pin_144"))]
            Uart2Tx::PD5(pa5) => { Self::build_cts(pa5, tx_dma, self.base, self.cts) }
        }
    }

    /// build by cts
    fn build_cts<TxDma>(
        tx: impl Peripheral<P=impl TxPin<USART2>> + 'static,
        tx_dma: impl Peripheral<P=TxDma> + 'static,
        base: UartBase<USART2>,
        cts: Option<Uart2Cts>)
        -> Result<UartTx<'static, USART2, TxDma>, ConfigError> {
        let cts = crate::match_some_return!(cts,
            UartTx::new(base.uart, tx, tx_dma, base.config.unwrap_or_default()));
        match cts {
            Uart2Cts::PA0(pa0) => { UartTx::new_with_cts(base.uart, tx, pa0, tx_dma, base.config.unwrap_or_default()) }
            #[cfg(any(feature = "pin_100", feature = "pin_144"))]
            Uart2Cts::PD3(pd3) => { UartTx::new_with_cts(base.uart, tx, pd3, tx_dma, base.config.unwrap_or_default()) }
        }
    }
}
