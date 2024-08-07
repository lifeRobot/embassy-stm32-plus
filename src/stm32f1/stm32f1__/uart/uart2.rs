#[macro_export]
macro_rules! mod_uart2 {
    () => {
        use embassy_stm32::{bind_interrupts, usart};
        use embassy_stm32::peripherals::{DMA1_CH6, DMA1_CH7, PA0, PA1, PA2, PA3, USART2};

        // uart irqs
        bind_interrupts!(pub struct Irqs {
            USART2 => usart::InterruptHandler<USART2>;
        });

        $crate::use_uart_trait!();

        // only DMA1_CH6 support read
        // only DMA1_CH7 support write
        $crate::impl_uart_ctsrts_trait!(USART2,PA2,PA3,PA1,PA0,DMA1_CH7,DMA1_CH6);
        $crate::impl_uart_ctsrts_trait!(USART2,PA2,PA3,PA1,PA0);
        $crate::impl_uart_trait!(USART2,PA2,PA3,DMA1_CH7,DMA1_CH6);
        $crate::impl_uart_trait!(USART2,PA2,PA3);

        // uart rx
        $crate::impl_uart_rx_rtscts_trait!(USART2,PA3,PA1,DMA1_CH6);
        $crate::impl_uart_rx_rtscts_trait!(USART2,PA3,PA1);
        $crate::impl_uart_rx_trait!(USART2,PA3,DMA1_CH6);
        $crate::impl_uart_rx_trait!(USART2,PA3);

        // uart tx
        $crate::impl_uart_tx_rtscts_trait!(USART2,PA2,PA0,DMA1_CH7);
        $crate::impl_uart_tx_rtscts_trait!(USART2,PA2,PA0);
        $crate::impl_uart_tx_trait!(USART2,PA2,DMA1_CH7);
        $crate::impl_uart_tx_trait!(USART2,PA2);
    };
}

#[macro_export]
macro_rules! modr_uart2 {
    () => {
        pub mod uart2 {
            $crate::mod_uart2!();
        }
    };
}

#[macro_export]
macro_rules! modz_uart2 {
    () => {
        pub mod uart2 {
            use embassy_stm32::peripherals::{PD3, PD4, PD5, PD6};

            $crate::mod_uart2!();

            // uart all
            $crate::impl_uart_ctsrts_trait!(USART2,PA2,PA3,PA1,PD3,DMA1_CH7,DMA1_CH6);
            $crate::impl_uart_ctsrts_trait!(USART2,PA2,PA3,PD4,PA0,DMA1_CH7,DMA1_CH6);
            $crate::impl_uart_ctsrts_trait!(USART2,PA2,PA3,PD4,PD3,DMA1_CH7,DMA1_CH6);
            $crate::impl_uart_ctsrts_trait!(USART2,PA2,PD6,PA1,PA0,DMA1_CH7,DMA1_CH6);
            $crate::impl_uart_ctsrts_trait!(USART2,PA2,PD6,PA1,PD3,DMA1_CH7,DMA1_CH6);
            $crate::impl_uart_ctsrts_trait!(USART2,PA2,PD6,PD4,PA0,DMA1_CH7,DMA1_CH6);
            $crate::impl_uart_ctsrts_trait!(USART2,PA2,PD6,PD4,PD3,DMA1_CH7,DMA1_CH6);
            $crate::impl_uart_ctsrts_trait!(USART2,PD5,PA3,PA1,PA0,DMA1_CH7,DMA1_CH6);
            $crate::impl_uart_ctsrts_trait!(USART2,PD5,PA3,PA1,PD3,DMA1_CH7,DMA1_CH6);
            $crate::impl_uart_ctsrts_trait!(USART2,PD5,PA3,PD4,PA0,DMA1_CH7,DMA1_CH6);
            $crate::impl_uart_ctsrts_trait!(USART2,PD5,PA3,PD4,PD3,DMA1_CH7,DMA1_CH6);
            $crate::impl_uart_ctsrts_trait!(USART2,PD5,PD6,PA1,PA0,DMA1_CH7,DMA1_CH6);
            $crate::impl_uart_ctsrts_trait!(USART2,PD5,PD6,PA1,PD3,DMA1_CH7,DMA1_CH6);
            $crate::impl_uart_ctsrts_trait!(USART2,PD5,PD6,PD4,PA0,DMA1_CH7,DMA1_CH6);
            $crate::impl_uart_ctsrts_trait!(USART2,PD5,PD6,PD4,PD3,DMA1_CH7,DMA1_CH6);

            $crate::impl_uart_ctsrts_trait!(USART2,PA2,PA3,PA1,PD3);
            $crate::impl_uart_ctsrts_trait!(USART2,PA2,PA3,PD4,PA0);
            $crate::impl_uart_ctsrts_trait!(USART2,PA2,PA3,PD4,PD3);
            $crate::impl_uart_ctsrts_trait!(USART2,PA2,PD6,PA1,PA0);
            $crate::impl_uart_ctsrts_trait!(USART2,PA2,PD6,PA1,PD3);
            $crate::impl_uart_ctsrts_trait!(USART2,PA2,PD6,PD4,PA0);
            $crate::impl_uart_ctsrts_trait!(USART2,PA2,PD6,PD4,PD3);
            $crate::impl_uart_ctsrts_trait!(USART2,PD5,PA3,PA1,PA0);
            $crate::impl_uart_ctsrts_trait!(USART2,PD5,PA3,PA1,PD3);
            $crate::impl_uart_ctsrts_trait!(USART2,PD5,PA3,PD4,PA0);
            $crate::impl_uart_ctsrts_trait!(USART2,PD5,PA3,PD4,PD3);
            $crate::impl_uart_ctsrts_trait!(USART2,PD5,PD6,PA1,PA0);
            $crate::impl_uart_ctsrts_trait!(USART2,PD5,PD6,PA1,PD3);
            $crate::impl_uart_ctsrts_trait!(USART2,PD5,PD6,PD4,PA0);
            $crate::impl_uart_ctsrts_trait!(USART2,PD5,PD6,PD4,PD3);

            $crate::impl_uart_trait!(USART2,PD5,PD6,DMA1_CH7,DMA1_CH6);
            $crate::impl_uart_trait!(USART2,PD5,PA3,DMA1_CH7,DMA1_CH6);
            $crate::impl_uart_trait!(USART2,PA2,PD6,DMA1_CH7,DMA1_CH6);

            $crate::impl_uart_trait!(USART2,PD5,PD6);
            $crate::impl_uart_trait!(USART2,PD5,PA3);
            $crate::impl_uart_trait!(USART2,PA2,PD6);

            // uart rx
            $crate::impl_uart_rx_rtscts_trait!(USART2,PA3,PD4,DMA1_CH6);
            $crate::impl_uart_rx_rtscts_trait!(USART2,PD6,PA1,DMA1_CH6);
            $crate::impl_uart_rx_rtscts_trait!(USART2,PD6,PD4,DMA1_CH6);
            $crate::impl_uart_rx_rtscts_trait!(USART2,PA3,PD4);
            $crate::impl_uart_rx_rtscts_trait!(USART2,PD6,PA1);
            $crate::impl_uart_rx_rtscts_trait!(USART2,PD6,PD4);

            $crate::impl_uart_rx_trait!(USART2,PD6,DMA1_CH6);
            $crate::impl_uart_rx_trait!(USART2,PD6);

            // uart tx
            $crate::impl_uart_tx_rtscts_trait!(USART2,PA2,PD3,DMA1_CH7);
            $crate::impl_uart_tx_rtscts_trait!(USART2,PD5,PA0,DMA1_CH7);
            $crate::impl_uart_tx_rtscts_trait!(USART2,PD5,PD3,DMA1_CH7);
            $crate::impl_uart_tx_rtscts_trait!(USART2,PA2,PD3);
            $crate::impl_uart_tx_rtscts_trait!(USART2,PD5,PA0);
            $crate::impl_uart_tx_rtscts_trait!(USART2,PD5,PD3);

            $crate::impl_uart_tx_trait!(USART2,PD5,DMA1_CH7);
            $crate::impl_uart_tx_trait!(USART2,PD5);
        }
    };
}
