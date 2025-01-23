#[cfg(any(USART1, USART2, USART3, UART4, UART5))]
pub mod uart;
#[cfg(any(CAN, CAN1, CAN2))]
pub mod can;