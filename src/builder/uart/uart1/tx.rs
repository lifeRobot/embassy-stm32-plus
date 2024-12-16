use embassy_stm32::dma::NoDma;
use embassy_stm32::Peripheral;
use embassy_stm32::peripherals::{DMA1_CH4, PA11, PA9, PB6, USART1};
use embassy_stm32::usart::{Config, ConfigError, TxPin, UartTx};
use crate::builder::uart::base::UartBase;

/// uart1 tx pin
pub enum Uart1Tx {
    PA9(PA9),
    PB6(PB6),
}

/// uart1 tx builder
pub struct Uart1TxBuilder {
    /// uart1 base data
    pub base: UartBase<USART1>,
    /// tx pin
    pub tx: Uart1Tx,
    /// use cts
    pub cts: Option<PA11>,
}

/// custom method
impl Uart1TxBuilder {
    /// create builder
    #[inline]
    pub fn new(uart: USART1, tx: Uart1Tx) -> Self {
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
    pub fn cts(mut self, cts: PA11) -> Self {
        self.cts = Some(cts);
        self
    }

    /// can not write
    #[inline]
    pub fn build_disable(self) -> Result<UartTx<'static, USART1>, ConfigError> {
        self.build_tx(NoDma)
    }

    /// build uart tx that supports write data
    #[inline]
    pub fn build_write(self, write_dma: DMA1_CH4) -> Result<UartTx<'static, USART1, DMA1_CH4>, ConfigError> {
        self.build_tx(write_dma)
    }

    /// build by tx
    fn build_tx<TxDma>(self, tx_dma: impl Peripheral<P=TxDma> + 'static) -> Result<UartTx<'static, USART1, TxDma>, ConfigError> {
        match self.tx {
            Uart1Tx::PA9(pa9) => { Self::build_cts(pa9, tx_dma, self.base, self.cts) }
            Uart1Tx::PB6(pb6) => { Self::build_cts(pb6, tx_dma, self.base, self.cts) }
        }
    }

    /// build cts or default
    fn build_cts<TxDma>(
        tx: impl Peripheral<P=impl TxPin<USART1>> + 'static,
        tx_dma: impl Peripheral<P=TxDma> + 'static,
        base: UartBase<USART1>,
        cts: Option<PA11>)
        -> Result<UartTx<'static, USART1, TxDma>, ConfigError> {
        let cts = crate::match_some_return!(cts,
            UartTx::new(base.uart, tx, tx_dma, base.config.unwrap_or_default()));
        UartTx::new_with_cts(base.uart, tx, cts, tx_dma, base.config.unwrap_or_default())
    }
}
