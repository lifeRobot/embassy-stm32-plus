/// build safe clone
#[macro_export]
macro_rules! build_safe_clone {
    () => {
        /// safe clone<br />
        /// only allowed for internal use, and there may be issues if reused
        pub(crate) trait SafeClone {
            fn clone(&self) -> Self;
        }

        /// support safe clone by any Peripheral
        impl<T: Peripheral<P=T>> SafeClone for T {
            fn clone(&self) -> Self {
                unsafe { self.clone_unchecked() }
            }
        }
    };
}


#[macro_export]
macro_rules! bind_eth {
    ($eth_trait_name:ident,$eth_fn_name:ident,$eth:ty,$ref_clk:ident,$mdio:ident,$mdc:ident,$crs:ident,$rx_d0:ident,$rx_d1:ident,$tx_d0:ident,$tx_d1:ident,$tx_en:ident,$phy:ty) => {
        pub trait $eth_trait_name: Peripheral + Instance {
            #[doc = "this function will transfer variables "]
            #[doc = stringify!($eth,$ref_clk,$mdio,$mdc,$crs,$rx_d0,$rx_d1,$tx_d0,$tx_d1,$tx_en.)]
            #[doc = " please do not use these variables again"]
            fn $eth_fn_name<'a, const TX: usize, const RX: usize>(&self, p: &Peripherals, queue: &'a mut PacketQueue<TX, RX>, phy: $phy, mac_addr: [u8; 6]) -> Ethernet<'a, Self, $phy>;
        }

        impl $eth_trait_name for $eth {
            fn $eth_fn_name<'a, const TX: usize, const RX: usize>(&self, p: &Peripherals, queue: &'a mut PacketQueue<TX, RX>, phy: $phy, mac_addr: [u8; 6]) -> Ethernet<'a, Self, $phy> {
                Ethernet::new(queue, self.clone(), Irqs, p.$ref_clk.clone(), p.$mdio.clone(), p.$mdc.clone(), p.$crs.clone(), p.$rx_d0.clone(), p.$rx_d1.clone(), p.$tx_d0.clone(), p.$tx_d1.clone(), p.$tx_en.clone(), phy, mac_addr)
            }
        }
    };
}

// support build eth simple call
crate::build_eth!();
