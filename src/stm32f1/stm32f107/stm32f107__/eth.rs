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
        $crate::bind_eth!(Eth2,eth2,ETH,PA1,PA2,PC1,PA7,PC4,PD10,PB12,PB13,PB11,GenericSMI);
        $crate::bind_eth!(Eth3,eth3,ETH,PA1,PA2,PC1,PA7,PD9,PC5 ,PB12,PB13,PB11,GenericSMI);
        $crate::bind_eth!(Eth4,eth4,ETH,PA1,PA2,PC1,PA7,PD9,PD10,PB12,PB13,PB11,GenericSMI);
        $crate::bind_eth!(Eth5,eth5,ETH,PA1,PA2,PC1,PD8,PC4,PC5 ,PB12,PB13,PB11,GenericSMI);
        $crate::bind_eth!(Eth6,eth6,ETH,PA1,PA2,PC1,PD8,PC4,PD10,PB12,PB13,PB11,GenericSMI);
        $crate::bind_eth!(Eth7,eth7,ETH,PA1,PA2,PC1,PD8,PD9,PC5 ,PB12,PB13,PB11,GenericSMI);
        $crate::bind_eth!(Eth8,eth8,ETH,PA1,PA2,PC1,PD8,PD9,PD10,PB12,PB13,PB11,GenericSMI);
    };
}

