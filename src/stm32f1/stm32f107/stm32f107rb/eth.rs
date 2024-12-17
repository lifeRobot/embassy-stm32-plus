/// build eth simple call
#[macro_export]
macro_rules! build_eth {
    () => {
        use embassy_stm32::{bind_interrupts, eth, Peripheral, Peripherals};
        use embassy_stm32::eth::{Ethernet, Instance, PacketQueue};
        use embassy_stm32::eth::generic_smi::GenericSMI;
        use embassy_stm32::peripherals::ETH;

        bind_interrupts!(pub struct Irqs {
            ETH => eth::InterruptHandler;
        });

        $crate::build_safe_clone!();
        $crate::bind_eth!(Eth1,eth1,ETH,PA1,PA2,PC1,PA7,PC4,PC5 ,PB12,PB13,PB11,GenericSMI);
    };
}

