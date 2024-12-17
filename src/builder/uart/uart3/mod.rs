use embassy_stm32::{bind_interrupts, Peripheral, usart};
use embassy_stm32::dma::NoDma;
use embassy_stm32::peripherals::{DMA1_CH2, DMA1_CH3, USART3};
use embassy_stm32::usart::{Config, ConfigError, RtsPin, RxPin, TxPin, Uart};
use crate::builder::uart::base::UartBase;
use crate::builder::uart::uart3::rx::{Uart3Rts, Uart3Rx, Uart3RxBuilder};
use crate::builder::uart::uart3::tx::{Uart3Cts, Uart3Tx};

pub mod rx;
pub mod tx;

bind_interrupts!(struct Irqs {
    USART3 => usart::InterruptHandler<USART3>;
});

/// uart3 builder
pub struct Uart3Builder {
    /// uart3 base data
    pub base: UartBase<USART3>,
    /// uart2 tx pin
    pub tx: Uart3Tx,
    /// uart2 rx pin
    pub rx: Uart3Rx,
    /// use rts cts
    pub rts_cts: Option<(Uart3Rts, Uart3Cts)>,
}

/// custom method
impl Uart3Builder {
    /// create builder
    #[inline]
    pub fn new(uart: USART3, tx: Uart3Tx, rx: Uart3Rx) -> Self {
        Self { base: UartBase::new(uart), tx, rx, rts_cts: None }
    }

    /// set uart config
    #[inline]
    pub fn config(mut self, config: Config) -> Self {
        self.base.set_config(config);
        self
    }

    /// set rts cts
    #[inline]
    pub fn rts_cts(mut self, rts: Uart3Rts, cts: Uart3Cts) -> Self {
        self.rts_cts = Some((rts, cts));
        self
    }

    /// can not read and write
    #[inline]
    pub fn build_disable(self) -> Result<Uart<'static, USART3, NoDma, NoDma>, ConfigError> {
        self.build_tx(NoDma, NoDma)
    }

    /// build a serial port that only supports read data
    #[inline]
    pub fn build_read(self, read_dma: DMA1_CH3) -> Result<Uart<'static, USART3, NoDma, DMA1_CH3>, ConfigError> {
        self.build_tx(NoDma, read_dma)
    }

    /// build a serial port that only supports write data
    #[inline]
    pub fn build_write(self, write_dma: DMA1_CH2) -> Result<Uart<'static, USART3, DMA1_CH2, NoDma>, ConfigError> {
        self.build_tx(write_dma, NoDma)
    }

    /// build a serial port that supports read and write data
    #[inline]
    pub fn build_all(self, write_dma: DMA1_CH2, read_dma: DMA1_CH3)
                     -> Result<Uart<'static, USART3, DMA1_CH2, DMA1_CH3>, ConfigError> {
        self.build_tx(write_dma, read_dma)
    }

    /// build by tx
    fn build_tx<TxDma, RxDma>(self, tx_dma: impl Peripheral<P=TxDma> + 'static, rx_dma: impl Peripheral<P=RxDma> + 'static)
                              -> Result<Uart<'static, USART3, TxDma, RxDma>, ConfigError> {
        let rx = Uart3RxBuilder { base: self.base, rx: self.rx, rts: None };
        match self.tx {
            Uart3Tx::PB10(pb10) => { Self::build_rx(pb10, rx, tx_dma, rx_dma, self.rts_cts) }
            Uart3Tx::PC10(pc10) => { Self::build_rx(pc10, rx, tx_dma, rx_dma, self.rts_cts) }
            #[cfg(any(feature = "pin_100", feature = "pin_144"))]
            Uart3Tx::PD8(pd8) => { Self::build_rx(pd8, rx, tx_dma, rx_dma, self.rts_cts) }
        }
    }

    /// build by rx
    fn build_rx<TxDma, RxDma>(
        tx: impl Peripheral<P=impl TxPin<USART3>> + 'static,
        rx: Uart3RxBuilder,
        tx_dma: impl Peripheral<P=TxDma> + 'static,
        rx_dma: impl Peripheral<P=RxDma> + 'static,
        rts_cts: Option<(Uart3Rts, Uart3Cts)>)
        -> Result<Uart<'static, USART3, TxDma, RxDma>, ConfigError> {
        match rx.rx {
            Uart3Rx::PB11(pb11) => { Self::build_rts(tx, pb11, rx.base, tx_dma, rx_dma, rts_cts) }
            Uart3Rx::PC11(pc11) => { Self::build_rts(tx, pc11, rx.base, tx_dma, rx_dma, rts_cts) }
            #[cfg(any(feature = "pin_100", feature = "pin_144"))]
            Uart3Rx::PD9(pd9) => { Self::build_rts(tx, pd9, rx.base, tx_dma, rx_dma, rts_cts) }
        }
    }

    /// build by rts
    fn build_rts<TxDma, RxDma>(
        tx: impl Peripheral<P=impl TxPin<USART3>> + 'static,
        rx: impl Peripheral<P=impl RxPin<USART3>> + 'static,
        base: UartBase<USART3>,
        tx_dma: impl Peripheral<P=TxDma> + 'static,
        rx_dma: impl Peripheral<P=RxDma> + 'static,
        rts_cts: Option<(Uart3Rts, Uart3Cts)>)
        -> Result<Uart<'static, USART3, TxDma, RxDma>, ConfigError> {
        let (rts, cts) = crate::match_some_return!(rts_cts,
            Uart::new(base.uart, rx, tx, Irqs, tx_dma, rx_dma, base.config.unwrap_or_default()));

        match rts {
            Uart3Rts::PB14(pb14) => { Self::build_cts(tx, rx, base, tx_dma, rx_dma, pb14, cts) }
            #[cfg(any(feature = "pin_100", feature = "pin_144"))]
            Uart3Rts::PD12(pd12) => { Self::build_cts(tx, rx, base, tx_dma, rx_dma, pd12, cts) }
        }
    }

    /// build by cts
    fn build_cts<TxDma, RxDma>(
        tx: impl Peripheral<P=impl TxPin<USART3>> + 'static,
        rx: impl Peripheral<P=impl RxPin<USART3>> + 'static,
        base: UartBase<USART3>,
        tx_dma: impl Peripheral<P=TxDma> + 'static,
        rx_dma: impl Peripheral<P=RxDma> + 'static,
        rts: impl Peripheral<P=impl RtsPin<USART3>> + 'static,
        cts: Uart3Cts)
        -> Result<Uart<'static, USART3, TxDma, RxDma>, ConfigError> {
        match cts {
            Uart3Cts::PB13(pb13) => {
                Uart::new_with_rtscts(base.uart, rx, tx, Irqs, rts, pb13, tx_dma, rx_dma, base.config.unwrap_or_default())
            }
            #[cfg(any(feature = "pin_100", feature = "pin_144"))]
            Uart3Cts::PD11(pd11) => {
                Uart::new_with_rtscts(base.uart, rx, tx, Irqs, rts, pd11, tx_dma, rx_dma, base.config.unwrap_or_default())
            }
        }
    }
}
