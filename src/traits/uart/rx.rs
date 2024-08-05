use embassy_stm32::dma::NoDma;
use embassy_stm32::Peripheral;
use embassy_stm32::usart::{BasicInstance, Config, UartRx};

/// all uart rx trait
pub trait UartRxAllTrait<Rx, Rts, RxDma = NoDma>: BasicInstance {
    fn build_rx_with_dma_rtscts_config(self, rx: Rx, rts: Rts, rx_dma: RxDma, config: Config) -> UartRx<'static, Self, RxDma>;

    fn build_rx_with_dma_rtscts(self, rx: Rx, rts: Rts, rx_dma: RxDma) -> UartRx<'static, Self, RxDma> {
        self.build_rx_with_dma_rtscts_config(rx, rts, rx_dma, Config::default())
    }
}

/// rts rx uart trait
pub trait UartRxRtsCtsTrait<Rx, Rts>: UartRxAllTrait<Rx, Rts> {
    fn build_rx_with_rtscts_config(self, rx: Rx, rts: Rts, config: Config) -> UartRx<'static, Self> {
        self.build_rx_with_dma_rtscts_config(rx, rts, NoDma, config)
    }

    fn build_rx_with_rtscts(self, rx: Rx, rts: Rts) -> UartRx<'static, Self> {
        self.build_rx_with_rtscts_config(rx, rts, Config::default())
    }
}

/// dma any uart rx trait
pub trait UartRxDmaAnyTrait<Rx>: BasicInstance {
    fn build_rx_with_dma_config_any<RxDma>(self, rx: Rx, rx_dma: impl Peripheral<P=RxDma> + 'static, config: Config) -> UartRx<'static, Self, RxDma>;

    fn build_rx_with_dma_any<RxDma>(self, rx: Rx, rx_dma: impl Peripheral<P=RxDma> + 'static) -> UartRx<'static, Self, RxDma> {
        self.build_rx_with_dma_config_any(rx, rx_dma, Config::default())
    }
}
/// dma uart rx trait
pub trait UartRxDmaTrait<Rx, RxDma = NoDma>: UartRxDmaAnyTrait<Rx> {
    fn build_rx_with_dma_config(self, rx: Rx, rx_dma: impl Peripheral<P=RxDma> + 'static, config: Config) -> UartRx<'static, Self, RxDma> {
        self.build_rx_with_dma_config_any(rx, rx_dma, config)
    }

    fn build_rx_with_dma(self, rx: Rx, rx_dma: impl Peripheral<P=RxDma> + 'static) -> UartRx<'static, Self, RxDma> {
        self.build_rx_with_dma_config(rx, rx_dma, Config::default())
    }
}

/// simple uart rx trait
pub trait UartRxTrait<Rx>: UartRxDmaTrait<Rx> {
    fn build_rx_with_config(self, rx: Rx, config: Config) -> UartRx<'static, Self> {
        self.build_rx_with_dma_config(rx, NoDma, config)
    }

    fn build_rx(self, rx: Rx) -> UartRx<'static, Self> {
        self.build_rx_with_config(rx, Config::default())
    }
}

/// impl uart rx trait
#[macro_export]
macro_rules! impl_uart_rx_trait {
    ($uart:ty,$rx:ty,$rx_dma:ty) => {
        impl UartRxDmaTrait<$rx, $rx_dma> for $uart {}
    };
    ($uart:ty,$rx:ty) => {
        $crate::impl_uart_rx_trait!($uart,$rx,NoDma);

        impl UartRxDmaAnyTrait<$rx> for $uart {
            fn build_rx_with_dma_config_any<RxDma>(self, rx: $rx, rx_dma: impl Peripheral<P=RxDma> + 'static, config: Config) -> UartRx<'static, Self, RxDma> {
                UartRx::new(self,Irqs,rx,rx_dma,config).expect("create uart rx fail")
            }
        }

        impl UartRxTrait<$rx> for $uart {}
    };
}

/// impl uart rx cts rts trait
#[macro_export]
macro_rules! impl_uart_rx_rtscts_trait {
    ($uart:ty,$rx:ty,$rts:ty,$rx_dma:ty) => {
        impl UartRxAllTrait<$rx,$rts,$rx_dma> for $uart {
            fn build_rx_with_dma_rtscts_config(self, rx: $rx, rts: $rts, rx_dma: $rx_dma, config: Config) -> UartRx<'static, Self, $rx_dma> {
                UartRx::new_with_rts(self,Irqs,rx,rts,rx_dma,config).expect("create uart rx fail")
            }
        }
    };
    ($uart:ty,$rx:ty,$rts:ty) => {
        crate::impl_uart_rx_rtscts_trait!($uart,$rx,$rts,NoDma);

        impl UartRxRtsCtsTrait<$rx,$rts> for $uart {}
    };
}
