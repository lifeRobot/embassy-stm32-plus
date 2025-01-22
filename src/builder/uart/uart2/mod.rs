use embassy_stm32::{bind_interrupts, Peripheral, usart};
use embassy_stm32::mode::Async;
use embassy_stm32::peripherals::{DMA1_CH6, DMA1_CH7, USART2};
use embassy_stm32::usart::{Config, ConfigError, RtsPin, RxDma, RxPin, TxDma, TxPin, Uart};
use crate::builder::uart::base::UartBase;
use crate::builder::uart::uart2::rx::{Uart2Rts, Uart2Rx, Uart2RxBuilder};
use crate::builder::uart::uart2::tx::{Uart2Cts, Uart2Tx};

pub mod rx;
pub mod tx;

bind_interrupts!(struct Irqs {
    USART2 => usart::InterruptHandler<USART2>;
});

/// uart2 builder
pub struct Uart2Builder {
    /// uart2 base data
    pub base: UartBase<USART2>,
    /// uart2 tx pin
    pub tx: Uart2Tx,
    /// uart2 rx pin
    pub rx: Uart2Rx,
    /// use rts cts
    pub rts_cts: Option<(Uart2Rts, Uart2Cts)>,
}

/// custom method
impl Uart2Builder {
    /// create builder
    #[inline]
    pub fn new(uart: USART2, tx: Uart2Tx, rx: Uart2Rx) -> Self {
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
    pub fn rts_cts(mut self, rts: Uart2Rts, cts: Uart2Cts) -> Self {
        self.rts_cts = Some((rts, cts));
        self
    }

    /*/// can not read and write
    #[inline]
    pub fn build_disable(self) -> Result<Uart<'static, Async>, ConfigError> {
        self.build_tx(NoDma, NoDma)
    }

    /// build a serial port that only supports read data
    #[inline]
    pub fn build_read(self, read_dma: DMA1_CH6) -> Result<Uart<'static, Async>, ConfigError> {
        self.build_tx(NoDma, read_dma)
    }

    /// build a serial port that only supports write data
    #[inline]
    pub fn build_write(self, write_dma: DMA1_CH7) -> Result<Uart<'static, Async>, ConfigError> {
        self.build_tx(write_dma, NoDma)
    }*/

    /// build a serial port that supports read and write data
    #[inline]
    pub fn build_all(self, write_dma: DMA1_CH7, read_dma: DMA1_CH6) -> Result<Uart<'static, Async>, ConfigError> {
        self.build_tx(write_dma, read_dma)
    }

    /// build by tx
    fn build_tx(self, tx_dma: impl Peripheral<P=impl TxDma<USART2>> + 'static, rx_dma: impl Peripheral<P=impl RxDma<USART2>> + 'static)
                -> Result<Uart<'static, Async>, ConfigError> {
        let rx = Uart2RxBuilder { base: self.base, rx: self.rx, rts: None };
        match self.tx {
            Uart2Tx::PA2(pa2) => { Self::build_rx(pa2, rx, tx_dma, rx_dma, self.rts_cts) }
            #[cfg(any(feature = "pin_100", feature = "pin_144"))]
            Uart2Tx::PD5(pd5) => { Self::build_rx(pd5, rx, tx_dma, rx_dma, self.rts_cts) }
        }
    }

    /// build by rx
    fn build_rx(
        tx: impl Peripheral<P=impl TxPin<USART2>> + 'static,
        rx: Uart2RxBuilder,
        tx_dma: impl Peripheral<P=impl TxDma<USART2>> + 'static,
        rx_dma: impl Peripheral<P=impl RxDma<USART2>> + 'static,
        rts_cts: Option<(Uart2Rts, Uart2Cts)>)
        -> Result<Uart<'static, Async>, ConfigError> {
        match rx.rx {
            Uart2Rx::PA3(pa3) => { Self::build_rts(tx, pa3, rx.base, tx_dma, rx_dma, rts_cts) }
            #[cfg(any(feature = "pin_100", feature = "pin_144"))]
            Uart2Rx::PD6(pd6) => { Self::build_rts(tx, pd6, rx.base, tx_dma, rx_dma, rts_cts) }
        }
    }

    /// build by rts
    fn build_rts(
        tx: impl Peripheral<P=impl TxPin<USART2>> + 'static,
        rx: impl Peripheral<P=impl RxPin<USART2>> + 'static,
        base: UartBase<USART2>,
        tx_dma: impl Peripheral<P=impl TxDma<USART2>> + 'static,
        rx_dma: impl Peripheral<P=impl RxDma<USART2>> + 'static,
        rts_cts: Option<(Uart2Rts, Uart2Cts)>)
        -> Result<Uart<'static, Async>, ConfigError> {
        let (rts, cts) = crate::match_some_return!(rts_cts,
            Uart::new(base.uart, rx, tx, Irqs, tx_dma, rx_dma, base.config.unwrap_or_default()));
        match rts {
            Uart2Rts::PA1(pa1) => { Self::build_cts(tx, rx, base, tx_dma, rx_dma, pa1, cts) }
            #[cfg(any(feature = "pin_100", feature = "pin_144"))]
            Uart2Rts::PD4(pd4) => { Self::build_cts(tx, rx, base, tx_dma, rx_dma, pd4, cts) }
        }
    }

    /// build by cts
    fn build_cts(
        tx: impl Peripheral<P=impl TxPin<USART2>> + 'static,
        rx: impl Peripheral<P=impl RxPin<USART2>> + 'static,
        base: UartBase<USART2>,
        tx_dma: impl Peripheral<P=impl TxDma<USART2>> + 'static,
        rx_dma: impl Peripheral<P=impl RxDma<USART2>> + 'static,
        rts: impl Peripheral<P=impl RtsPin<USART2>> + 'static,
        cts: Uart2Cts)
        -> Result<Uart<'static, Async>, ConfigError> {
        match cts {
            Uart2Cts::PA0(pa0) => {
                Uart::new_with_rtscts(base.uart, rx, tx, Irqs, rts, pa0, tx_dma, rx_dma, base.config.unwrap_or_default())
            }
            #[cfg(any(feature = "pin_100", feature = "pin_144"))]
            Uart2Cts::PD3(pd3) => {
                Uart::new_with_rtscts(base.uart, rx, tx, Irqs, rts, pd3, tx_dma, rx_dma, base.config.unwrap_or_default())
            }
        }
    }
}
