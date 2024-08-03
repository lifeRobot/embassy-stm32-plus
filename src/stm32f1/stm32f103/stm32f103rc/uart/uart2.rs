use embassy_stm32::{bind_interrupts, usart};
use embassy_stm32::peripherals::{DMA1_CH6, DMA1_CH7, PA0, PA1, PA2, PA3, USART2};

// uart irqs
bind_interrupts!(pub struct Irqs {
    USART2 => usart::InterruptHandler<USART2>;
});

crate::use_uart_trait!();

// only DMA1_CH6 support read
// only DMA1_CH7 support write
crate::impl_uart_ctsrts_trait!(USART2,PA2,PA3,PA1,PA0,DMA1_CH7,DMA1_CH6);
crate::impl_uart_ctsrts_trait!(USART2,PA2,PA3,PA1,PA0);
crate::impl_uart_trait!(USART2,PA2,PA3,DMA1_CH7,DMA1_CH6);
crate::impl_uart_trait!(USART2,PA2,PA3);
