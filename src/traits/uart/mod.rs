#[cfg(USART1)]
pub mod uart1;
#[cfg(USART2)]
pub mod uart2;
#[cfg(USART3)]
pub mod uart3;
#[cfg(all(
    UART4,
    not(any(
        feature = "stm32f100rc",
        feature = "stm32f100rd",
        feature = "stm32f100re",
        feature = "stm32f100vc",
        feature = "stm32f100vd",
        feature = "stm32f100ve",
        feature = "stm32f100zc",
        feature = "stm32f100zd",
        feature = "stm32f100ze",
    ))
))]
pub mod uart4;
#[cfg(all(
    UART5,
    not(any(
        feature = "stm32f100rc",
        feature = "stm32f100rd",
        feature = "stm32f100re",
        feature = "stm32f100vc",
        feature = "stm32f100vd",
        feature = "stm32f100ve",
        feature = "stm32f100zc",
        feature = "stm32f100zd",
        feature = "stm32f100ze",
    ))
))]
pub mod uart5;