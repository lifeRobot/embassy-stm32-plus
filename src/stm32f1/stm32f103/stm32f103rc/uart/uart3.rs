use embassy_stm32::{bind_interrupts, usart};
use embassy_stm32::peripherals::{DMA1_CH2, DMA1_CH3, PB10, PB11, PB13, PB14, PC10, PC11, USART3};

bind_interrupts!(pub struct Irqs {
    USART3 => usart::InterruptHandler<USART3>;
});

crate::use_uart_trait!();

// only DMA1_CH3 support read
// only DMA1_CH2 support write
crate::impl_uart_ctsrts_trait!(USART3,PB10,PB11,PB14,PB13,DMA1_CH2,DMA1_CH3);
crate::impl_uart_ctsrts_trait!(USART3,PB10,PC11,PB14,PB13,DMA1_CH2,DMA1_CH3);
crate::impl_uart_ctsrts_trait!(USART3,PC10,PC11,PB14,PB13,DMA1_CH2,DMA1_CH3);
crate::impl_uart_ctsrts_trait!(USART3,PC10,PB11,PB14,PB13,DMA1_CH2,DMA1_CH3);

crate::impl_uart_ctsrts_trait!(USART3,PB10,PB11,PB14,PB13);
crate::impl_uart_ctsrts_trait!(USART3,PB10,PC11,PB14,PB13);
crate::impl_uart_ctsrts_trait!(USART3,PC10,PC11,PB14,PB13);
crate::impl_uart_ctsrts_trait!(USART3,PC10,PB11,PB14,PB13);

crate::impl_uart_trait!(USART3,PB10,PB11,DMA1_CH2,DMA1_CH3);
crate::impl_uart_trait!(USART3,PB10,PC11,DMA1_CH2,DMA1_CH3);
crate::impl_uart_trait!(USART3,PC10,PC11,DMA1_CH2,DMA1_CH3);
crate::impl_uart_trait!(USART3,PC10,PB11,DMA1_CH2,DMA1_CH3);

crate::impl_uart_trait!(USART3,PB10,PB11);
crate::impl_uart_trait!(USART3,PB10,PC11);
crate::impl_uart_trait!(USART3,PC10,PC11);
crate::impl_uart_trait!(USART3,PC10,PB11);
