use embassy_stm32::Peripheral;
use embassy_stm32::can::{Can, TxPin};
#[cfg(CAN)]
use embassy_stm32::peripherals::CAN;
#[cfg(CAN1)]
use embassy_stm32::peripherals::CAN1;
use embassy_stm32::peripherals::{PA11, PA12};
#[cfg(PB8)]
use embassy_stm32::peripherals::PB8;
#[cfg(PB9)]
use embassy_stm32::peripherals::PB9;
#[cfg(any(CAN_PD0, CAN1_PD0))]
use embassy_stm32::peripherals::PD0;
#[cfg(any(CAN_PD1, CAN1_PD1))]
use embassy_stm32::peripherals::PD1;
use crate::irq_s::usb_can1::Irqs;

/// can1 rx pin
pub enum Can1Rx {
    PA11(PA11),
    #[cfg(PB8)]
    PB8(PB8),
    #[cfg(any(CAN_PD0, CAN1_PD0))]
    PD0(PD0),
}

/// can1 tx pin
pub enum Can1Tx {
    PA12(PA12),
    #[cfg(PB9)]
    PB9(PB9),
    #[cfg(any(CAN_PD1, CAN1_PD1))]
    PD1(PD1),
}

/// can1 builder
pub struct Can1Builder {
    /// can device
    #[cfg(CAN)]
    pub can: CAN,
    /// can1 device
    #[cfg(CAN1)]
    pub can: CAN1,
    /// can1 tx pin
    pub tx: Can1Tx,
    /// can1 rx pin
    pub rx: Can1Rx,
}

macro_rules! build_rx_fn {
    ($can:ty) => {
        /// build by rx
        fn build_rx(can: $can, tx: impl Peripheral<P=impl TxPin<$can>> + 'static, rx: Can1Rx) -> Can<'static> {
            match rx {
                Can1Rx::PA11(pa11) => { Can::new(can, pa11, tx, Irqs) }
                #[cfg(PB8)]
                Can1Rx::PB8(pb8) => { Can::new(can, pb8, tx, Irqs) }
                #[cfg(any(CAN_PD0, CAN1_PD0))]
                Can1Rx::PD0(pd0) => { Can::new(can, pd0, tx, Irqs) }
            }
        }
    };
}

/// custom method
impl Can1Builder {
    /// create builder
    #[cfg(CAN)]
    #[inline]
    pub fn new(can: CAN, tx: Can1Tx, rx: Can1Rx) -> Self {
        Self { can, tx, rx }
    }

    /// create builder
    #[cfg(CAN1)]
    #[inline]
    pub fn new(can: CAN1, tx: Can1Tx, rx: Can1Rx) -> Self {
        Self { can, tx, rx }
    }

    /// build bx_can instance, more see [Can::new]
    pub fn build(self) -> Can<'static> {
        match self.tx {
            Can1Tx::PA12(pa12) => { Self::build_rx(self.can, pa12, self.rx) }
            #[cfg(PB9)]
            Can1Tx::PB9(pb9) => { Self::build_rx(self.can, pb9, self.rx) }
            #[cfg(any(CAN_PD1, CAN1_PD1))]
            Can1Tx::PD1(pd1) => { Self::build_rx(self.can, pd1, self.rx) }
        }
    }

    #[cfg(CAN)]
    build_rx_fn!(CAN);
    #[cfg(CAN1)]
    build_rx_fn!(CAN1);
}
