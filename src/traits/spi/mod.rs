use embassy_stm32::dma::NoDma;
use embassy_stm32::Peripheral;
use embassy_stm32::spi::{Config, Instance, Spi};

/// spi dma any trait
pub trait SpiDmaAnyTrait<SCK, MOSI, MISO>: Instance {
    fn build_with_dma_config_any<TxDma, RxDma>(self, sck: SCK, mosi: MOSI, miso: MISO, tx_dma: impl Peripheral<P=TxDma> + 'static, rx_dma: impl Peripheral<P=RxDma> + 'static, config: Config) -> Spi<'static, Self, TxDma, RxDma>;

    fn build_rxonly_with_dma_config_any<TxDma, RxDma>(self, sck: SCK, miso: MISO, tx_dma: impl Peripheral<P=TxDma> + 'static, rx_dma: impl Peripheral<P=RxDma> + 'static, config: Config) -> Spi<'static, Self, TxDma, RxDma>;

    fn build_txonly_with_dma_config_any<TxDma, RxDma>(self, sck: SCK, mosi: MOSI, tx_dma: impl Peripheral<P=TxDma> + 'static, rx_dma: impl Peripheral<P=RxDma> + 'static, config: Config) -> Spi<'static, Self, TxDma, RxDma>;

    fn build_txonly_nosck_with_dma_config_any<TxDma, RxDma>(self, mosi: MOSI, tx_dma: impl Peripheral<P=TxDma> + 'static, rx_dma: impl Peripheral<P=RxDma> + 'static, config: Config) -> Spi<'static, Self, TxDma, RxDma>;

    fn build_with_dma_any<TxDma, RxDma>(self, sck: SCK, mosi: MOSI, miso: MISO, tx_dma: impl Peripheral<P=TxDma> + 'static, rx_dma: impl Peripheral<P=RxDma> + 'static) -> Spi<'static, Self, TxDma, RxDma> {
        self.build_with_dma_config_any(sck, mosi, miso, tx_dma, rx_dma, Config::default())
    }

    fn build_rxonly_with_dma_any<TxDma, RxDma>(self, sck: SCK, miso: MISO, tx_dma: impl Peripheral<P=TxDma> + 'static, rx_dma: impl Peripheral<P=RxDma> + 'static) -> Spi<'static, Self, TxDma, RxDma> {
        self.build_rxonly_with_dma_config_any(sck, miso, tx_dma, rx_dma, Config::default())
    }

    fn build_txonly_with_dma_any<TxDma, RxDma>(self, sck: SCK, mosi: MOSI, tx_dma: impl Peripheral<P=TxDma> + 'static, rx_dma: impl Peripheral<P=RxDma> + 'static) -> Spi<'static, Self, TxDma, RxDma> {
        self.build_txonly_with_dma_config_any(sck, mosi, tx_dma, rx_dma, Config::default())
    }

    fn build_txonly_nosck_with_dma_any<TxDma, RxDma>(self, mosi: MOSI, tx_dma: impl Peripheral<P=TxDma> + 'static, rx_dma: impl Peripheral<P=RxDma> + 'static) -> Spi<'static, Self, TxDma, RxDma> {
        self.build_txonly_nosck_with_dma_config_any(mosi, tx_dma, rx_dma, Config::default())
    }
}

/// spi dma trait
pub trait SpiDmaTrait<SCK, MOSI, MISO, TxDma = NoDma, RxDma = NoDma>: SpiDmaAnyTrait<SCK, MOSI, MISO> {
    fn build_with_dma_config(self, sck: SCK, mosi: MOSI, miso: MISO, tx_dma: impl Peripheral<P=TxDma> + 'static, rx_dma: impl Peripheral<P=RxDma> + 'static, config: Config) -> Spi<'static, Self, TxDma, RxDma> {
        self.build_with_dma_config_any(sck, mosi, miso, tx_dma, rx_dma, config)
    }

    fn build_with_dma(self, sck: SCK, mosi: MOSI, miso: MISO, tx_dma: impl Peripheral<P=TxDma> + 'static, rx_dma: impl Peripheral<P=RxDma> + 'static) -> Spi<'static, Self, TxDma, RxDma> {
        self.build_with_dma_config(sck, mosi, miso, tx_dma, rx_dma, Config::default())
    }

    fn build_rxonly_with_dma_config(self, sck: SCK, miso: MISO, tx_dma: impl Peripheral<P=TxDma> + 'static, rx_dma: impl Peripheral<P=RxDma> + 'static, config: Config) -> Spi<'static, Self, TxDma, RxDma> {
        self.build_rxonly_with_dma_config_any(sck, miso, tx_dma, rx_dma, config)
    }

    fn build_rxonly_with_dma(self, sck: SCK, miso: MISO, tx_dma: impl Peripheral<P=TxDma> + 'static, rx_dma: impl Peripheral<P=RxDma> + 'static) -> Spi<'static, Self, TxDma, RxDma> {
        self.build_rxonly_with_dma_config(sck, miso, tx_dma, rx_dma, Config::default())
    }

    fn build_txonly_with_dma_config(self, sck: SCK, mosi: MOSI, tx_dma: impl Peripheral<P=TxDma> + 'static, rx_dma: impl Peripheral<P=RxDma> + 'static, config: Config) -> Spi<'static, Self, TxDma, RxDma> {
        self.build_txonly_with_dma_config_any(sck, mosi, tx_dma, rx_dma, config)
    }

    fn build_txonly_with_dma(self, sck: SCK, mosi: MOSI, tx_dma: impl Peripheral<P=TxDma> + 'static, rx_dma: impl Peripheral<P=RxDma> + 'static) -> Spi<'static, Self, TxDma, RxDma> {
        self.build_txonly_with_dma_config(sck, mosi, tx_dma, rx_dma, Config::default())
    }

    fn build_txonly_nosck_with_dma_config(self, mosi: MOSI, tx_dma: impl Peripheral<P=TxDma> + 'static, rx_dma: impl Peripheral<P=RxDma> + 'static, config: Config) -> Spi<'static, Self, TxDma, RxDma> {
        self.build_txonly_nosck_with_dma_config_any(mosi, tx_dma, rx_dma, config)
    }

    fn build_txonly_nosck_with_dma(self, mosi: MOSI, tx_dma: impl Peripheral<P=TxDma> + 'static, rx_dma: impl Peripheral<P=RxDma> + 'static) -> Spi<'static, Self, TxDma, RxDma> {
        self.build_txonly_nosck_with_dma_config(mosi, tx_dma, rx_dma, Config::default())
    }
}

/// spi trait
pub trait SpiTrait<SCK, MOSI, MISO>: SpiDmaTrait<SCK, MOSI, MISO> {
    fn build_with_config(self, sck: SCK, mosi: MOSI, miso: MISO, config: Config) -> Spi<'static, Self, NoDma, NoDma> {
        self.build_with_dma_config(sck, mosi, miso, NoDma, NoDma, config)
    }

    fn build(self, sck: SCK, mosi: MOSI, miso: MISO) -> Spi<'static, Self, NoDma, NoDma> {
        self.build_with_config(sck, mosi, miso, Config::default())
    }

    fn build_rxonly_with_config(self, sck: SCK, miso: MISO, config: Config) -> Spi<'static, Self, NoDma, NoDma> {
        self.build_rxonly_with_dma_config(sck, miso, NoDma, NoDma, config)
    }

    fn build_rxonly_with(self, sck: SCK, miso: MISO) -> Spi<'static, Self, NoDma, NoDma> {
        self.build_rxonly_with_config(sck, miso, Config::default())
    }

    fn build_txonly_with_config(self, sck: SCK, mosi: MOSI, config: Config) -> Spi<'static, Self, NoDma, NoDma> {
        self.build_txonly_with_dma_config(sck, mosi, NoDma, NoDma, config)
    }

    fn build_txonly_with(self, sck: SCK, mosi: MOSI) -> Spi<'static, Self, NoDma, NoDma> {
        self.build_txonly_with_config(sck, mosi, Config::default())
    }

    fn build_txonly_nosck_with_config(self, mosi: MOSI, config: Config) -> Spi<'static, Self, NoDma, NoDma> {
        self.build_txonly_nosck_with_dma_config(mosi, NoDma, NoDma, config)
    }

    fn build_txonly_nosck_with(self, mosi: MOSI) -> Spi<'static, Self, NoDma, NoDma> {
        self.build_txonly_nosck_with_config(mosi, Config::default())
    }
}

/// use spi trait
#[macro_export]
macro_rules! use_spi_trait {
    () => {
        use $crate::traits::spi::{SpiDmaAnyTrait, SpiDmaTrait, SpiTrait};
        use embassy_stm32::dma::NoDma;
        use embassy_stm32::Peripheral;
        use embassy_stm32::spi::{Config, Spi};
    };
}

/// impl spi trait
#[macro_export]
macro_rules! impl_spi_trait {
    ($spi:ty,$sck:ty,$mosi:ty,$miso:ty,$tx_dma:ty,$rx_dma:ty) => {
        impl SpiDmaTrait<$sck,$mosi,$miso,$tx_dma,$rx_dma> for $spi {}
    };
    ($spi:ty,$sck:ty,$mosi:ty,$miso:ty) => {
        $crate::impl_spi_trait!($spi,$sck,$mosi,$miso,NoDma,NoDma);

        impl SpiDmaAnyTrait<$sck,$mosi,$miso> for $spi {
            fn build_with_dma_config_any<TxDma, RxDma>(self, sck: $sck, mosi: $mosi, miso: $miso, tx_dma: impl Peripheral<P=TxDma> + 'static, rx_dma: impl Peripheral<P=RxDma> + 'static, config: Config) -> Spi<'static, Self, TxDma, RxDma> {
                Spi::new(self,sck,mosi,miso,tx_dma,rx_dma,config)
            }

            fn build_rxonly_with_dma_config_any<TxDma, RxDma>(self, sck: $sck, miso: $miso, tx_dma: impl Peripheral<P=TxDma> + 'static, rx_dma: impl Peripheral<P=RxDma> + 'static, config: Config) -> Spi<'static, Self, TxDma, RxDma> {
                Spi::new_rxonly(self,sck,miso,tx_dma,rx_dma,config)
            }

            fn build_txonly_with_dma_config_any<TxDma, RxDma>(self, sck: $sck, mosi: $mosi, tx_dma: impl Peripheral<P=TxDma> + 'static, rx_dma: impl Peripheral<P=RxDma> + 'static, config: Config) -> Spi<'static, Self, TxDma, RxDma> {
                Spi::new_txonly(self,sck,mosi,tx_dma,rx_dma,config)
            }

            fn build_txonly_nosck_with_dma_config_any<TxDma, RxDma>(self, mosi: $mosi, tx_dma: impl Peripheral<P=TxDma> + 'static, rx_dma: impl Peripheral<P=RxDma> + 'static, config: Config) -> Spi<'static, Self, TxDma, RxDma>{
                Spi::new_txonly_nosck(self,mosi,tx_dma,rx_dma,config)
            }
        }

        impl SpiTrait<$sck,$mosi,$miso> for $spi {}
    };
}
