use embassy_stm32::{bind_interrupts, can, Peripheral};
use embassy_stm32::can::{Can, TxPin};
use embassy_stm32::peripherals::{CAN2, PB12, PB13, PB5, PB6};

bind_interrupts!(struct Irqs {
    CAN2_RX0 => can::Rx0InterruptHandler<CAN2>;
    CAN2_RX1 => can::Rx1InterruptHandler<CAN2>;
    CAN2_SCE => can::SceInterruptHandler<CAN2>;
    CAN2_TX => can::TxInterruptHandler<CAN2>;
});

/// can2 rx pin
pub enum Can2Rx {
    PB5(PB5),
    PB12(PB12),
}

/// can2 tx pin
pub enum Can2Tx {
    PB6(PB6),
    PB13(PB13),
}

/// can2 builder
pub struct Can2Builder {
    /// can2 device
    pub can: CAN2,
    /// can2 tx pin
    pub tx: Can2Tx,
    /// can2 rx pin
    pub rx: Can2Rx,
}

/// custom method
impl Can2Builder {
    /// create builder
    #[inline]
    pub fn new(can: CAN2, tx: Can2Tx, rx: Can2Rx) -> Self {
        Self { can, tx, rx }
    }

    /// build bx_can instance, more see [Can::new]
    pub fn build(self) -> Can<'static> {
        match self.tx {
            Can2Tx::PB6(pb6) => { Self::build_rx(self.can, pb6, self.rx) }
            Can2Tx::PB13(pb13) => { Self::build_rx(self.can, pb13, self.rx) }
        }
    }

    /// build by rx
    fn build_rx(can: CAN2, tx: impl Peripheral<P=impl TxPin<CAN2>> + 'static, rx: Can2Rx) -> Can<'static> {
        match rx {
            Can2Rx::PB5(pb5) => { Can::new(can, pb5, tx, Irqs) }
            Can2Rx::PB12(pb12) => { Can::new(can, pb12, tx, Irqs) }
        }
    }
}
