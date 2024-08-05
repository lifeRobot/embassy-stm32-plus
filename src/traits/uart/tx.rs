use embassy_stm32::dma::NoDma;
use embassy_stm32::Peripheral;
use embassy_stm32::usart::{BasicInstance, Config, UartTx};

/// all uart tx trait
pub trait UartTxAllTrait<Tx, Cts, TxDma = NoDma>: BasicInstance {
    fn build_tx_with_dma_rtscts_config(self, tx: Tx, cts: Cts, tx_dma: TxDma, config: Config) -> UartTx<'static, Self, TxDma>;

    fn build_tx_with_dma_rtscts(self, tx: Tx, cts: Cts, tx_dma: TxDma) -> UartTx<'static, Self, TxDma> {
        self.build_tx_with_dma_rtscts_config(tx, cts, tx_dma, Config::default())
    }
}

/// rts tx uart trait
pub trait UartTxRtsCtsTrait<Tx, Cts>: UartTxAllTrait<Tx, Cts> {
    fn build_tx_with_rtscts_config(self, tx: Tx, cts: Cts, config: Config) -> UartTx<'static, Self> {
        self.build_tx_with_dma_rtscts_config(tx, cts, NoDma, config)
    }

    fn build_tx_with_rtscts(self, tx: Tx, cts: Cts) -> UartTx<'static, Self> {
        self.build_tx_with_rtscts_config(tx, cts, Config::default())
    }
}

/// dma any uart tx trait
pub trait UartTxDmaAnyTrait<Tx>: BasicInstance {
    fn build_tx_with_dma_config_any<TxDma>(self, tx: Tx, tx_dma: impl Peripheral<P=TxDma> + 'static, config: Config) -> UartTx<'static, Self, TxDma>;

    fn build_tx_with_dma_any<TxDma>(self, tx: Tx, tx_dma: impl Peripheral<P=TxDma> + 'static) -> UartTx<'static, Self, TxDma> {
        self.build_tx_with_dma_config_any(tx, tx_dma, Config::default())
    }
}

/// dma uart tx trait
pub trait UartTxDmaTrait<Tx, TxDma = NoDma>: UartTxDmaAnyTrait<Tx> {
    fn build_tx_with_dma_config(self, tx: Tx, tx_dma: impl Peripheral<P=TxDma> + 'static, config: Config) -> UartTx<'static, Self, TxDma> {
        self.build_tx_with_dma_config_any(tx, tx_dma, config)
    }

    fn build_tx_with_dma(self, tx: Tx, tx_dma: impl Peripheral<P=TxDma> + 'static) -> UartTx<'static, Self, TxDma> {
        self.build_tx_with_dma_config(tx, tx_dma, Config::default())
    }
}
/// simple uart tx trait
pub trait UartTxTrait<Tx>: UartTxDmaTrait<Tx> {
    fn build_tx_with_config(self, tx: Tx, config: Config) -> UartTx<'static, Self> {
        self.build_tx_with_dma_config(tx, NoDma, config)
    }

    fn build_tx_with(self, tx: Tx) -> UartTx<'static, Self> {
        self.build_tx_with_config(tx, Config::default())
    }
}

/// impl uart tx trait
#[macro_export]
macro_rules! impl_uart_tx_trait {
    ($uart:ty,$tx:ty,$tx_dma:ty) => {
        impl UartTxDmaTrait<$tx, $tx_dma> for $uart {}
    };
    ($uart:ty,$tx:ty) => {
        crate::impl_uart_tx_trait!($uart,$tx,NoDma);

        impl UartTxDmaAnyTrait<$tx> for $uart {
            fn build_tx_with_dma_config_any<TxDma>(self, tx: $tx, tx_dma: impl Peripheral<P=TxDma> + 'static, config: Config) -> UartTx<'static, Self, TxDma> {
                UartTx::new(self,tx,tx_dma,config).expect("create uart tx fail")
            }
        }

        impl UartTxTrait<$tx> for $uart {}
    };
}

/// impl uart tx cts rts trait
#[macro_export]
macro_rules! impl_uart_tx_rtscts_trait {
    ($uart:ty,$tx:ty,$cts:ty,$tx_dma:ty) => {
        impl UartTxAllTrait<$tx,$cts,$tx_dma> for $uart {
            fn build_tx_with_dma_rtscts_config(self, tx: $tx, cts: $cts, tx_dma: $tx_dma, config: Config) -> UartTx<'static, Self, $tx_dma> {
                UartTx::new_with_cts(self,tx,cts,tx_dma,config).expect("create uart tx fail")
            }
        }
    };
    ($uart:ty,$tx:ty,$cts:ty) => {
        crate::impl_uart_tx_rtscts_trait!($uart,$tx,$cts,NoDma);

        impl UartTxRtsCtsTrait<$tx,$cts> for $uart {}
    };
}
