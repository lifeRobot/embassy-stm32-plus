/// build f103 uart1
#[macro_export]
macro_rules! f103_uart1 {
    () => {
        pub mod uart1 {
            use embassy_stm32::{bind_interrupts, usart};
            use embassy_stm32::peripherals::{DMA1_CH4, DMA1_CH5, PA10, PA11, PA12, PA9, PB6, PB7, USART1};

            // uart irqs
            bind_interrupts!(pub struct Irqs {
                USART1 => usart::InterruptHandler<USART1>;
            });

            $crate::use_uart_trait!();

            // only DMA1_CH5 support read
            // only DMA1_CH4 support write
            $crate::impl_uart_ctsrts_trait!(USART1,PA9,PA10,PA12,PA11,DMA1_CH4,DMA1_CH5);
            $crate::impl_uart_ctsrts_trait!(USART1,PA9,PB7,PA12,PA11,DMA1_CH4,DMA1_CH5);
            $crate::impl_uart_ctsrts_trait!(USART1,PB6,PB7,PA12,PA11,DMA1_CH4,DMA1_CH5);
            $crate::impl_uart_ctsrts_trait!(USART1,PB6,PA10,PA12,PA11,DMA1_CH4,DMA1_CH5);

            $crate::impl_uart_ctsrts_trait!(USART1,PA9,PA10,PA12,PA11);
            $crate::impl_uart_ctsrts_trait!(USART1,PA9,PB7,PA12,PA11);
            $crate::impl_uart_ctsrts_trait!(USART1,PB6,PB7,PA12,PA11);
            $crate::impl_uart_ctsrts_trait!(USART1,PB6,PA10,PA12,PA11);

            $crate::impl_uart_trait!(USART1,PA9,PA10,DMA1_CH4,DMA1_CH5);
            $crate::impl_uart_trait!(USART1,PA9,PB7,DMA1_CH4,DMA1_CH5);
            $crate::impl_uart_trait!(USART1,PB6,PB7,DMA1_CH4,DMA1_CH5);
            $crate::impl_uart_trait!(USART1,PB6,PA10,DMA1_CH4,DMA1_CH5);

            $crate::impl_uart_trait!(USART1,PA9,PA10);
            $crate::impl_uart_trait!(USART1,PA9,PB7);
            $crate::impl_uart_trait!(USART1,PB6,PB7);
            $crate::impl_uart_trait!(USART1,PB6,PA10);

            // uart rx
            $crate::impl_uart_rx_rtscts_trait!(USART1,PA10,PA12,DMA1_CH5);
            $crate::impl_uart_rx_rtscts_trait!(USART1,PB7,PA12,DMA1_CH5);
            $crate::impl_uart_rx_rtscts_trait!(USART1,PA10,PA12);
            $crate::impl_uart_rx_rtscts_trait!(USART1,PB7,PA12);

            $crate::impl_uart_rx_trait!(USART1,PA10,DMA1_CH5);
            $crate::impl_uart_rx_trait!(USART1,PB7,DMA1_CH5);
            $crate::impl_uart_rx_trait!(USART1,PA10);
            $crate::impl_uart_rx_trait!(USART1,PB7);

            // uart tx
            $crate::impl_uart_tx_rtscts_trait!(USART1,PA9,PA11,DMA1_CH4);
            $crate::impl_uart_tx_rtscts_trait!(USART1,PB6,PA11,DMA1_CH4);
            $crate::impl_uart_tx_rtscts_trait!(USART1,PA9,PA11);
            $crate::impl_uart_tx_rtscts_trait!(USART1,PB6,PA11);

            $crate::impl_uart_tx_trait!(USART1,PA9,DMA1_CH4);
            $crate::impl_uart_tx_trait!(USART1,PB6,DMA1_CH4);
            $crate::impl_uart_tx_trait!(USART1,PA9);
            $crate::impl_uart_tx_trait!(USART1,PB6);
        }
    };
}