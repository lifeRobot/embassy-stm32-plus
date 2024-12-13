#[macro_export]
macro_rules! impl_uart3 {
    () => {
        use embassy_stm32::{bind_interrupts, usart};
        use embassy_stm32::peripherals::{DMA1_CH2, DMA1_CH3, PB10, PB11, PB13, PB14, USART3};

        bind_interrupts!(pub struct Irqs {
            USART3 => usart::InterruptHandler<USART3>;
        });

        $crate::use_uart_trait!();

        // only DMA1_CH3 support read
        // only DMA1_CH2 support write
        $crate::impl_uart_ctsrts_trait!(USART3,PB10,PB11,PB14,PB13,DMA1_CH2,DMA1_CH3);
        $crate::impl_uart_ctsrts_trait!(USART3,PB10,PB11,PB14,PB13);
        $crate::impl_uart_trait!(USART3,PB10,PB11,DMA1_CH2,DMA1_CH3);
        $crate::impl_uart_trait!(USART3,PB10,PB11);


        // uart rx
        $crate::impl_uart_rx_rtscts_trait!(USART3,PB11,PB14,DMA1_CH3);
        $crate::impl_uart_rx_rtscts_trait!(USART3,PB11,PB14);
        $crate::impl_uart_rx_trait!(USART3,PB11,DMA1_CH3);
        $crate::impl_uart_rx_trait!(USART3,PB11);

        // uart tx
        $crate::impl_uart_tx_rtscts_trait!(USART3,PB10,PB13,DMA1_CH2);
        $crate::impl_uart_tx_rtscts_trait!(USART3,PB10,PB13);
        $crate::impl_uart_tx_trait!(USART3,PB10,DMA1_CH2);
        $crate::impl_uart_tx_trait!(USART3,PB10);
    };
}

#[macro_export]
macro_rules! implr_uart3 {
    () => {
        use embassy_stm32::peripherals::{PC10, PC11};

        $crate::impl_uart3!();

        // uart all
        $crate::impl_uart_ctsrts_trait!(USART3,PB10,PC11,PB14,PB13,DMA1_CH2,DMA1_CH3);
        $crate::impl_uart_ctsrts_trait!(USART3,PC10,PC11,PB14,PB13,DMA1_CH2,DMA1_CH3);
        $crate::impl_uart_ctsrts_trait!(USART3,PC10,PB11,PB14,PB13,DMA1_CH2,DMA1_CH3);

        $crate::impl_uart_ctsrts_trait!(USART3,PB10,PC11,PB14,PB13);
        $crate::impl_uart_ctsrts_trait!(USART3,PC10,PC11,PB14,PB13);
        $crate::impl_uart_ctsrts_trait!(USART3,PC10,PB11,PB14,PB13);

        $crate::impl_uart_trait!(USART3,PB10,PC11,DMA1_CH2,DMA1_CH3);
        $crate::impl_uart_trait!(USART3,PC10,PC11,DMA1_CH2,DMA1_CH3);
        $crate::impl_uart_trait!(USART3,PC10,PB11,DMA1_CH2,DMA1_CH3);

        $crate::impl_uart_trait!(USART3,PB10,PC11);
        $crate::impl_uart_trait!(USART3,PC10,PC11);
        $crate::impl_uart_trait!(USART3,PC10,PB11);

        // uart rx
        $crate::impl_uart_rx_rtscts_trait!(USART3,PC11,PB14,DMA1_CH3);
        $crate::impl_uart_rx_rtscts_trait!(USART3,PC11,PB14);
        $crate::impl_uart_rx_trait!(USART3,PC11,DMA1_CH3);
        $crate::impl_uart_rx_trait!(USART3,PC11);

        // uart tx
        $crate::impl_uart_tx_rtscts_trait!(USART3,PC10,PB13,DMA1_CH2);
        $crate::impl_uart_tx_rtscts_trait!(USART3,PC10,PB13);
        $crate::impl_uart_tx_trait!(USART3,PC10,DMA1_CH2);
        $crate::impl_uart_tx_trait!(USART3,PC10);
    };
}

#[macro_export]
macro_rules! modc_uart3 {
    () => {
        pub mod uart3 {
            $crate::impl_uart3!();
        }
    };
}

#[macro_export]
macro_rules! modr_uart3 {
    () => {
        pub mod uart3 {
            $crate::implr_uart3!();
        }
    }
}

#[macro_export]
macro_rules! modz_uart3 {
    () => {
        pub mod uart3 {
            use embassy_stm32::peripherals::{PD11, PD12, PD8, PD9};

            $crate::implr_uart3!();

            // uart all
            $crate::impl_uart_ctsrts_trait!(USART3,PB10,PB11,PB14,PD11,DMA1_CH2,DMA1_CH3);
            $crate::impl_uart_ctsrts_trait!(USART3,PB10,PB11,PD12,PB13,DMA1_CH2,DMA1_CH3);
            $crate::impl_uart_ctsrts_trait!(USART3,PB10,PB11,PD12,PD11,DMA1_CH2,DMA1_CH3);
            $crate::impl_uart_ctsrts_trait!(USART3,PB10,PC11,PB14,PD11,DMA1_CH2,DMA1_CH3);
            $crate::impl_uart_ctsrts_trait!(USART3,PB10,PC11,PD12,PB13,DMA1_CH2,DMA1_CH3);
            $crate::impl_uart_ctsrts_trait!(USART3,PB10,PC11,PD12,PD11,DMA1_CH2,DMA1_CH3);
            $crate::impl_uart_ctsrts_trait!(USART3,PB10,PD9,PB14,PB13,DMA1_CH2,DMA1_CH3);
            $crate::impl_uart_ctsrts_trait!(USART3,PB10,PD9,PB14,PD11,DMA1_CH2,DMA1_CH3);
            $crate::impl_uart_ctsrts_trait!(USART3,PB10,PD9,PD12,PB13,DMA1_CH2,DMA1_CH3);
            $crate::impl_uart_ctsrts_trait!(USART3,PB10,PD9,PD12,PD11,DMA1_CH2,DMA1_CH3);
            $crate::impl_uart_ctsrts_trait!(USART3,PC10,PB11,PB14,PD11,DMA1_CH2,DMA1_CH3);
            $crate::impl_uart_ctsrts_trait!(USART3,PC10,PB11,PD12,PB13,DMA1_CH2,DMA1_CH3);
            $crate::impl_uart_ctsrts_trait!(USART3,PC10,PB11,PD12,PD11,DMA1_CH2,DMA1_CH3);
            $crate::impl_uart_ctsrts_trait!(USART3,PC10,PC11,PB14,PD11,DMA1_CH2,DMA1_CH3);
            $crate::impl_uart_ctsrts_trait!(USART3,PC10,PC11,PD12,PB13,DMA1_CH2,DMA1_CH3);
            $crate::impl_uart_ctsrts_trait!(USART3,PC10,PC11,PD12,PD11,DMA1_CH2,DMA1_CH3);
            $crate::impl_uart_ctsrts_trait!(USART3,PC10,PD9,PB14,PB13,DMA1_CH2,DMA1_CH3);
            $crate::impl_uart_ctsrts_trait!(USART3,PC10,PD9,PB14,PD11,DMA1_CH2,DMA1_CH3);
            $crate::impl_uart_ctsrts_trait!(USART3,PC10,PD9,PD12,PB13,DMA1_CH2,DMA1_CH3);
            $crate::impl_uart_ctsrts_trait!(USART3,PC10,PD9,PD12,PD11,DMA1_CH2,DMA1_CH3);
            $crate::impl_uart_ctsrts_trait!(USART3,PD8,PB11,PB14,PD11,DMA1_CH2,DMA1_CH3);
            $crate::impl_uart_ctsrts_trait!(USART3,PD8,PB11,PD12,PB13,DMA1_CH2,DMA1_CH3);
            $crate::impl_uart_ctsrts_trait!(USART3,PD8,PB11,PD12,PD11,DMA1_CH2,DMA1_CH3);
            $crate::impl_uart_ctsrts_trait!(USART3,PD8,PC11,PB14,PD11,DMA1_CH2,DMA1_CH3);
            $crate::impl_uart_ctsrts_trait!(USART3,PD8,PC11,PD12,PB13,DMA1_CH2,DMA1_CH3);
            $crate::impl_uart_ctsrts_trait!(USART3,PD8,PC11,PD12,PD11,DMA1_CH2,DMA1_CH3);
            $crate::impl_uart_ctsrts_trait!(USART3,PD8,PD9,PB14,PB13,DMA1_CH2,DMA1_CH3);
            $crate::impl_uart_ctsrts_trait!(USART3,PD8,PD9,PB14,PD11,DMA1_CH2,DMA1_CH3);
            $crate::impl_uart_ctsrts_trait!(USART3,PD8,PD9,PD12,PB13,DMA1_CH2,DMA1_CH3);
            $crate::impl_uart_ctsrts_trait!(USART3,PD8,PD9,PD12,PD11,DMA1_CH2,DMA1_CH3);
            $crate::impl_uart_ctsrts_trait!(USART3,PD8,PC11,PB14,PB13,DMA1_CH2,DMA1_CH3);
            $crate::impl_uart_ctsrts_trait!(USART3,PD8,PB11,PB14,PB13,DMA1_CH2,DMA1_CH3);

            $crate::impl_uart_ctsrts_trait!(USART3,PB10,PB11,PB14,PD11);
            $crate::impl_uart_ctsrts_trait!(USART3,PB10,PB11,PD12,PB13);
            $crate::impl_uart_ctsrts_trait!(USART3,PB10,PB11,PD12,PD11);
            $crate::impl_uart_ctsrts_trait!(USART3,PB10,PC11,PB14,PD11);
            $crate::impl_uart_ctsrts_trait!(USART3,PB10,PC11,PD12,PB13);
            $crate::impl_uart_ctsrts_trait!(USART3,PB10,PC11,PD12,PD11);
            $crate::impl_uart_ctsrts_trait!(USART3,PB10,PD9,PB14,PB13);
            $crate::impl_uart_ctsrts_trait!(USART3,PB10,PD9,PB14,PD11);
            $crate::impl_uart_ctsrts_trait!(USART3,PB10,PD9,PD12,PB13);
            $crate::impl_uart_ctsrts_trait!(USART3,PB10,PD9,PD12,PD11);
            $crate::impl_uart_ctsrts_trait!(USART3,PC10,PB11,PB14,PD11);
            $crate::impl_uart_ctsrts_trait!(USART3,PC10,PB11,PD12,PB13);
            $crate::impl_uart_ctsrts_trait!(USART3,PC10,PB11,PD12,PD11);
            $crate::impl_uart_ctsrts_trait!(USART3,PC10,PC11,PB14,PD11);
            $crate::impl_uart_ctsrts_trait!(USART3,PC10,PC11,PD12,PB13);
            $crate::impl_uart_ctsrts_trait!(USART3,PC10,PC11,PD12,PD11);
            $crate::impl_uart_ctsrts_trait!(USART3,PC10,PD9,PB14,PB13);
            $crate::impl_uart_ctsrts_trait!(USART3,PC10,PD9,PB14,PD11);
            $crate::impl_uart_ctsrts_trait!(USART3,PC10,PD9,PD12,PB13);
            $crate::impl_uart_ctsrts_trait!(USART3,PC10,PD9,PD12,PD11);
            $crate::impl_uart_ctsrts_trait!(USART3,PD8,PB11,PB14,PD11);
            $crate::impl_uart_ctsrts_trait!(USART3,PD8,PB11,PD12,PB13);
            $crate::impl_uart_ctsrts_trait!(USART3,PD8,PB11,PD12,PD11);
            $crate::impl_uart_ctsrts_trait!(USART3,PD8,PC11,PB14,PD11);
            $crate::impl_uart_ctsrts_trait!(USART3,PD8,PC11,PD12,PB13);
            $crate::impl_uart_ctsrts_trait!(USART3,PD8,PC11,PD12,PD11);
            $crate::impl_uart_ctsrts_trait!(USART3,PD8,PD9,PB14,PB13);
            $crate::impl_uart_ctsrts_trait!(USART3,PD8,PD9,PB14,PD11);
            $crate::impl_uart_ctsrts_trait!(USART3,PD8,PD9,PD12,PB13);
            $crate::impl_uart_ctsrts_trait!(USART3,PD8,PD9,PD12,PD11);
            $crate::impl_uart_ctsrts_trait!(USART3,PD8,PC11,PB14,PB13);
            $crate::impl_uart_ctsrts_trait!(USART3,PD8,PB11,PB14,PB13);


            $crate::impl_uart_trait!(USART3,PB10,PD9,DMA1_CH2,DMA1_CH3);
            $crate::impl_uart_trait!(USART3,PC10,PD9,DMA1_CH2,DMA1_CH3);
            $crate::impl_uart_trait!(USART3,PD8,PD9,DMA1_CH2,DMA1_CH3);
            $crate::impl_uart_trait!(USART3,PD8,PB11,DMA1_CH2,DMA1_CH3);
            $crate::impl_uart_trait!(USART3,PD8,PC11,DMA1_CH2,DMA1_CH3);

            $crate::impl_uart_trait!(USART3,PB10,PD9);
            $crate::impl_uart_trait!(USART3,PC10,PD9);
            $crate::impl_uart_trait!(USART3,PD8,PD9);
            $crate::impl_uart_trait!(USART3,PD8,PB11);
            $crate::impl_uart_trait!(USART3,PD8,PC11);

            // uart rx
            $crate::impl_uart_rx_rtscts_trait!(USART3,PD9,PB14,DMA1_CH3);
            $crate::impl_uart_rx_rtscts_trait!(USART3,PD9,PD12,DMA1_CH3);
            $crate::impl_uart_rx_rtscts_trait!(USART3,PB11,PD12,DMA1_CH3);
            $crate::impl_uart_rx_rtscts_trait!(USART3,PC11,PD12,DMA1_CH3);
            $crate::impl_uart_rx_rtscts_trait!(USART3,PD9,PB14);
            $crate::impl_uart_rx_rtscts_trait!(USART3,PD9,PD12);
            $crate::impl_uart_rx_rtscts_trait!(USART3,PB11,PD12);
            $crate::impl_uart_rx_rtscts_trait!(USART3,PC11,PD12);

            $crate::impl_uart_rx_trait!(USART3,PD9,DMA1_CH3);
            $crate::impl_uart_rx_trait!(USART3,PD9);

            // uart tx
            $crate::impl_uart_tx_rtscts_trait!(USART3,PD8,PB13,DMA1_CH2);
            $crate::impl_uart_tx_rtscts_trait!(USART3,PD8,PD11,DMA1_CH2);
            $crate::impl_uart_tx_rtscts_trait!(USART3,PB10,PD11,DMA1_CH2);
            $crate::impl_uart_tx_rtscts_trait!(USART3,PC10,PD11,DMA1_CH2);
            $crate::impl_uart_tx_rtscts_trait!(USART3,PD8,PB13);
            $crate::impl_uart_tx_rtscts_trait!(USART3,PD8,PD11);
            $crate::impl_uart_tx_rtscts_trait!(USART3,PB10,PD11);
            $crate::impl_uart_tx_rtscts_trait!(USART3,PC10,PD11);

            $crate::impl_uart_tx_trait!(USART3,PD8,DMA1_CH2);
            $crate::impl_uart_tx_trait!(USART3,PD8);
        }
    };
}
