### goal

Make Rust Embedded simpler

### support now

- stm32f103xx(uart/gpio/flash) &#10004;
- more support comming soon

### uart example

Cargo.toml file :

```toml
embassy-stm32-plus = { version = "0.1.1", features = ["stm32f103rc"] }
embassy-executor = { version = "0.6.0", features = ["arch-cortex-m", "executor-thread", "defmt", "integrated-timers"] }
embassy-time = { version = "0.3.2", features = ["defmt", "defmt-timestamp-uptime", "tick-hz-32_768"] }
defmt-rtt = "0.4.1"
cortex-m = { version = "0.7.7", features = ["inline-asm", "critical-section-single-core"] }
cortex-m-rt = "0.7.3"
panic-probe = { version = "0.3.2", features = ["print-defmt"] }
```

main.rs file :

```rust
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32_plus::embassy_stm32;
use embassy_stm32_plus::traits::uart::UartDmaTrait;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // init stm32, get Peripheral
    let p = embassy_stm32::init(Default::default());

    // simple init uart
    // let uart = p.USART3.build_with_dma(p.PB10, p.PB11, p.DMA1_CH2, p.DMA1_CH3);
    // let uart = p.USART2.build_with_dma(p.PA2, p.PA3, p.DMA1_CH7, p.DMA1_CH6);
    let uart = p.USART1.build_with_dma(p.PA9, p.PA10, p.DMA1_CH4, p.DMA1_CH5);
    let (mut tx, _rx) = uart.split();

    // let mut b = [0; 1024];
    // rx.read(&mut b).await.unwrap();

    // send data to uart
    loop {
        tx.write(b"hello world").await.unwrap();
        Timer::after_millis(1000).await;
    }
}
```

### gpio example
Cargo.toml: see [uart example]
```rust
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32_plus::embassy_stm32;
use embassy_stm32_plus::traits::gpio::output::GpioOutput;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // init stm32, get Peripheral
    let p = embassy_stm32::init(Default::default());

    // simple get output/input gpio
    let mut led = p.PA8.output();

    // change gpio level
    loop {
        led.set_high();
        Timer::after_millis(300).await;

        led.set_low();
        Timer::after_millis(300).await;
    }
}
```

### Other instructions

- build linrary(.bin): `cargo objcopy --release -- -O binary app.bin`
- build ihex(.hex): `cargo objcopy --release -- -O ihex app.hex`
- debug see [probe-rs](https://probe.rs/)
- more see [embassy](https://github.com/embassy-rs/embassy)

