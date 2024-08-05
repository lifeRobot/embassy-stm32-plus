use embassy_stm32::dma::NoDma;
use embassy_stm32::Peripheral;
use embassy_stm32::usart::{BasicInstance, Config, Uart};

pub mod rx;
pub mod tx;

/// all uart trait
pub trait UartAllTrait<Tx, Rx, Rts, Cts, TxDma = NoDma, RxDma = NoDma>: BasicInstance {
    fn build_with_dma_rtscts_config(self, tx: Tx, rx: Rx, tx_dma: TxDma, rx_dma: RxDma, rts: Rts, cts: Cts, config: Config) -> Uart<'static, Self, TxDma, RxDma>;

    fn build_with_dma_rtscts(self, tx: Tx, rx: Rx, tx_dma: TxDma, rx_dma: RxDma, rts: Rts, cts: Cts) -> Uart<'static, Self, TxDma, RxDma> {
        self.build_with_dma_rtscts_config(tx, rx, tx_dma, rx_dma, rts, cts, Config::default())
    }
}

/// rtscts uart trait
pub trait UartRtsCtsTrait<Tx, Rx, Rts, Cts>: UartAllTrait<Tx, Rx, Rts, Cts> {
    fn build_with_rtscts_config(self, tx: Tx, rx: Rx, rts: Rts, cts: Cts, config: Config) -> Uart<'static, Self> {
        self.build_with_dma_rtscts_config(tx, rx, NoDma, NoDma, rts, cts, config)
    }

    fn build_with_rtscts(self, tx: Tx, rx: Rx, rts: Rts, cts: Cts) -> Uart<'static, Self> {
        self.build_with_rtscts_config(tx, rx, rts, cts, Config::default())
    }
}

/// dma any uart trait
pub trait UartDmaAnyTrait<Tx, Rx>: BasicInstance {
    fn build_with_dma_config_any<TxDma, RxDma>(self, tx: Tx, rx: Rx, tx_dma: impl Peripheral<P=TxDma> + 'static, rx_dma: impl Peripheral<P=RxDma> + 'static, config: Config) -> Uart<'static, Self, TxDma, RxDma>;

    fn build_with_dma_any<TxDma, RxDma>(self, tx: Tx, rx: Rx, tx_dma: impl Peripheral<P=TxDma> + 'static, rx_dma: impl Peripheral<P=RxDma> + 'static) -> Uart<'static, Self, TxDma, RxDma> {
        self.build_with_dma_config_any(tx, rx, tx_dma, rx_dma, Config::default())
    }
}

/// dma uart trait
pub trait UartDmaTrait<Tx, Rx, TxDma = NoDma, RxDma = NoDma>: UartDmaAnyTrait<Tx, Rx> {
    fn build_with_dma_config(self, tx: Tx, rx: Rx, tx_dma: impl Peripheral<P=TxDma> + 'static, rx_dma: impl Peripheral<P=RxDma> + 'static, config: Config) -> Uart<'static, Self, TxDma, RxDma> {
        self.build_with_dma_config_any(tx, rx, tx_dma, rx_dma, config)
    }

    fn build_with_dma(self, tx: Tx, rx: Rx, tx_dma: impl Peripheral<P=TxDma> + 'static, rx_dma: impl Peripheral<P=RxDma> + 'static) -> Uart<'static, Self, TxDma, RxDma> {
        self.build_with_dma_config(tx, rx, tx_dma, rx_dma, Config::default())
    }
}

/// simple uart trait
pub trait UartTrait<Tx, Rx>: UartDmaTrait<Tx, Rx> {
    fn build_with_config(self, tx: Tx, rx: Rx, config: Config) -> Uart<'static, Self> {
        self.build_with_dma_config(tx, rx, NoDma, NoDma, config)
    }

    fn build(self, tx: Tx, rx: Rx) -> Uart<'static, Self> {
        self.build_with_config(tx, rx, Config::default())
    }
}

/// use uart trait
#[macro_export]
macro_rules! use_uart_trait {
    () => {
        use embassy_stm32::usart::{Config, Uart, UartRx, UartTx};
        use embassy_stm32::Peripheral;
        use embassy_stm32::dma::NoDma;
        use $crate::traits::uart::{UartAllTrait, UartDmaAnyTrait, UartDmaTrait, UartRtsCtsTrait, UartTrait};
        use $crate::traits::uart::rx::{UartRxAllTrait, UartRxDmaAnyTrait, UartRxDmaTrait, UartRxRtsCtsTrait, UartRxTrait};
        use $crate::traits::uart::tx::{UartTxAllTrait, UartTxDmaAnyTrait, UartTxDmaTrait, UartTxRtsCtsTrait, UartTxTrait};
    };
}

/// impl uart trait
#[macro_export]
macro_rules! impl_uart_trait {
    ($uart:ty,$tx:ty,$rx:ty,$tx_dma:ty,$rx_dma:ty) => {
        impl UartDmaTrait<$tx, $rx, $tx_dma, $rx_dma> for $uart {}
    };
    ($uart:ty,$tx:ty,$rx:ty) => {
        $crate::impl_uart_trait!($uart,$tx,$rx,NoDma,NoDma);

        impl UartDmaAnyTrait<$tx,$rx> for $uart {
            fn build_with_dma_config_any<TxDma, RxDma>(self, tx: $tx, rx: $rx, tx_dma: impl Peripheral<P=TxDma> + 'static, rx_dma: impl Peripheral<P=RxDma> + 'static, config: Config) -> Uart<'static, Self, TxDma, RxDma> {
                Uart::new(self, rx, tx, Irqs, tx_dma, rx_dma, config).expect("create usart fail")
            }
        }

        impl UartTrait<$tx, $rx> for $uart {}
    };
}

/// impl uart cts rts trait
#[macro_export]
macro_rules! impl_uart_ctsrts_trait {
    ($uart:ty,$tx:ty,$rx:ty,$rts:ty,$cts:ty,$tx_dma:ty,$rx_dma:ty) => {
        impl UartAllTrait<$tx, $rx, $rts, $cts, $tx_dma, $rx_dma> for $uart {
            fn build_with_dma_rtscts_config(self, tx: $tx, rx: $rx, tx_dma: $tx_dma, rx_dma: $rx_dma, rts: $rts, cts: $cts, config: Config) -> Uart<'static, Self, $tx_dma, $rx_dma> {
                Uart::new_with_rtscts(self, rx, tx, Irqs, rts, cts, tx_dma, rx_dma, config).expect("create usart fail")
            }
        }
    };
    ($uart:ty,$tx:ty,$rx:ty,$rts:ty,$cts:ty) => {
        $crate::impl_uart_ctsrts_trait!($uart,$tx, $rx, $rts, $cts,NoDma,NoDma);

        impl UartRtsCtsTrait<$tx, $rx, $rts, $cts> for $uart {}
    };
}
