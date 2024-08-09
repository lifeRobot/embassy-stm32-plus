use embassy_stm32::{bind_interrupts, eth, Peripheral, Peripherals};
use embassy_stm32::eth::{Ethernet, Instance, PacketQueue};
use embassy_stm32::eth::generic_smi::GenericSMI;
use embassy_stm32::peripherals::ETH;
use crate::traits::eth::SafeClone;

bind_interrupts!(pub struct Irqs {
    ETH => eth::InterruptHandler;
});

// TODO the code here may be migrated
crate::bind_eth!(Eth1,eth1,ETH,PA1,PA2,PC1,PA7,PC4,PC5,PB12,PB13,PB11,GenericSMI);
crate::bind_eth!(Eth2,eth2,ETH,PA1,PA2,PC1,PD8,PD9,PD10,PB12,PB13,PB11,GenericSMI);
