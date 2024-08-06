use embassy_stm32::Peripheral;
use embassy_stm32::dma::NoDma;
use embassy_stm32::i2c::{Config, I2c, Instance};
use embassy_stm32::time::Hertz;

/// i2c any dam trait
pub trait I2cDmaAnyTrait<SCL, SDA>: Peripheral + Instance {
    fn build_with_dma_config_any<TxDma, RxDma>(self, scl: SCL, sda: SDA, tx_dma: impl Peripheral<P=TxDma> + 'static, rx_dma: impl Peripheral<P=RxDma> + 'static, freq: Hertz, config: Config) -> I2c<'static, Self, TxDma, RxDma>;

    fn build_with_dma_any<TxDma, RxDma>(self, scl: SCL, sda: SDA, tx_dma: impl Peripheral<P=TxDma> + 'static, rx_dma: impl Peripheral<P=RxDma> + 'static, freq: Hertz) -> I2c<'static, Self, TxDma, RxDma> {
        self.build_with_dma_config_any(scl, sda, tx_dma, rx_dma, freq, Config::default())
    }
}

/// i2c dma trait
pub trait I2cDmaTrait<SCL, SDA, TxDma = NoDma, RxDma = NoDma>: I2cDmaAnyTrait<SCL, SDA> {
    fn build_with_dma_config(self, scl: SCL, sda: SDA, tx_dma: impl Peripheral<P=TxDma> + 'static, rx_dma: impl Peripheral<P=RxDma> + 'static, freq: Hertz, config: Config) -> I2c<'static, Self, TxDma, RxDma> {
        self.build_with_dma_config_any(scl, sda, tx_dma, rx_dma, freq, config)
    }

    fn build_with_dma(self, scl: SCL, sda: SDA, tx_dma: impl Peripheral<P=TxDma> + 'static, rx_dma: impl Peripheral<P=RxDma> + 'static, freq: Hertz) -> I2c<'static, Self, TxDma, RxDma> {
        self.build_with_dma_config(scl, sda, tx_dma, rx_dma, freq, Config::default())
    }
}

/// i2c trait
pub trait I2cTrait<SCL, SDA>: I2cDmaTrait<SCL, SDA> {
    fn build_with_config(self, scl: SCL, sda: SDA, freq: Hertz, config: Config) -> I2c<'static, Self> {
        self.build_with_dma_config(scl, sda, NoDma, NoDma, freq, config)
    }

    fn build(self, scl: SCL, sda: SDA, freq: Hertz) -> I2c<'static, Self> {
        self.build_with_config(scl, sda, freq, Config::default())
    }
}

/// use i2c trait
#[macro_export]
macro_rules! use_i2c_trait {
    () => {
        use $crate::traits::i2c::{I2cTrait, I2cDmaAnyTrait, I2cDmaTrait};
        use embassy_stm32::Peripheral;
        use embassy_stm32::dma::NoDma;
        use embassy_stm32::i2c::{Config, I2c};
        use embassy_stm32::time::Hertz;
    };
}

/// impl i2c trait
#[macro_export]
macro_rules! impl_i2c_trait {
    ($i2c:ty,$scl:ty,$sda:ty,$tx_dma:ty,$rx_dma:ty) => {
        impl I2cDmaTrait<$scl,$sda,$tx_dma,$rx_dma> for $i2c {}
    };
    ($i2c:ty,$scl:ty,$sda:ty) => {
        $crate::impl_i2c_trait!($i2c,$scl,$sda,NoDma,NoDma);

        impl I2cDmaAnyTrait<$scl,$sda> for $i2c {
            fn build_with_dma_config_any<TxDma, RxDma>(self, scl: $scl, sda: $sda, tx_dma: impl Peripheral<P=TxDma> + 'static, rx_dma: impl Peripheral<P=RxDma> + 'static, freq: Hertz, config: Config) -> I2c<'static, Self, TxDma, RxDma> {
                I2c::new(self,scl,sda,Irqs,tx_dma,rx_dma,freq,config)
            }
        }

        impl I2cTrait<$scl,$sda> for $i2c {}
    };
}
