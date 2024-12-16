use embassy_stm32::{bind_interrupts, usart};
use embassy_stm32::peripherals::USART2;

pub mod rx;
pub mod tx;

bind_interrupts!(struct Irqs {
    USART2 => usart::InterruptHandler<USART2>;
});

/// uart2 builder
pub struct Uart2Builder {}