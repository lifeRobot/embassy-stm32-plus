use embassy_stm32::dma::NoDma;
use embassy_stm32::Peripheral;
use embassy_stm32::peripherals::{DMA1_CH2, PB10, PB13, PC10, USART3};
#[cfg(any(feature = "pin_100", feature = "pin_144"))]
use embassy_stm32::peripherals::{PD11, PD8};
use embassy_stm32::usart::{Config, ConfigError, TxPin, UartTx};
use crate::builder::uart::base::UartBase;

/// uart3 tx pin
pub enum Uart3Tx {
    PB10(PB10),
    PC10(PC10),
    #[cfg(any(feature = "pin_100", feature = "pin_144"))]
    PD8(PD8),
}

/// uart3 cts pin
pub enum Uart3Cts {
    PB13(PB13),
    #[cfg(any(feature = "pin_100", feature = "pin_144"))]
    PD11(PD11),
}

/// uart3 tx builder
pub struct Uart3TxBuilder {
    /// uart2 base data
    pub base: UartBase<USART3>,
    /// tx pin
    pub tx: Uart3Tx,
    /// use cts
    pub cts: Option<Uart3Cts>,
}

/// custom method
impl Uart3TxBuilder {
    /// create builder
    #[inline]
    pub fn new(uart: USART3, tx: Uart3Tx) -> Self {
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
    pub fn cts(mut self, cts: Uart3Cts) -> Self {
        self.cts = Some(cts);
        self
    }

    /// can not write
    #[inline]
    pub fn build_disable(self) -> Result<UartTx<'static, USART3, NoDma>, ConfigError> {
        self.build_tx(NoDma)
    }

    /// build uart tx that supports write data
    #[inline]
    pub fn build_write(self, write_dma: DMA1_CH2) -> Result<UartTx<'static, USART3, DMA1_CH2>, ConfigError> {
        self.build_tx(write_dma)
    }

    /// build by tx
    fn build_tx<TxDma>(self, tx_dma: impl Peripheral<P=TxDma> + 'static) -> Result<UartTx<'static, USART3, TxDma>, ConfigError> {
        match self.tx {
            Uart3Tx::PB10(pb10) => { Self::build_cts(pb10, tx_dma, self.base, self.cts) }
            Uart3Tx::PC10(pc10) => { Self::build_cts(pc10, tx_dma, self.base, self.cts) }
            #[cfg(any(feature = "pin_100", feature = "pin_144"))]
            Uart3Tx::PD8(pd8) => { Self::build_cts(pd8, tx_dma, self.base, self.cts) }
        }
    }

    /// build by cts
    fn build_cts<TxDma>(
        tx: impl Peripheral<P=impl TxPin<USART3>> + 'static,
        tx_dma: impl Peripheral<P=TxDma> + 'static,
        base: UartBase<USART3>,
        cts: Option<Uart3Cts>)
        -> Result<UartTx<'static, USART3, TxDma>, ConfigError> {
        let cts = crate::match_some_return!(cts,
            UartTx::new(base.uart, tx, tx_dma, base.config.unwrap_or_default()));

        match cts {
            Uart3Cts::PB13(pb13) => { UartTx::new_with_cts(base.uart, tx, pb13, tx_dma, base.config.unwrap_or_default()) }
            #[cfg(any(feature = "pin_100", feature = "pin_144"))]
            Uart3Cts::PD11(pd11) => { UartTx::new_with_cts(base.uart, tx, pd11, tx_dma, base.config.unwrap_or_default()) }
        }
    }
}
