### goal

Make Rust Embedded simpler

### support now

- stm32f103xx &#10004;
- more support comming soon

### example

<details open>
<summary>uart example</summary>

Cargo.toml file :

```toml
embassy-stm32-plus = { version = "0.1.2", features = ["stm32f103rc"] }
embassy-executor = { version = "0.6.0", features = ["arch-cortex-m", "executor-thread", "defmt", "integrated-timers"] }
defmt-rtt = "0.4.1"
cortex-m-rt = "0.7.3"
panic-probe = { version = "0.3.2", features = ["print-defmt"] }
```

main.rs file :

```rust
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32_plus::embassy_stm32;
use embassy_stm32_plus::embassy_time::Timer;
use embassy_stm32_plus::traits::uart::UartDmaTrait;
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

</details>

<details>
<summary>gpio example</summary>

Cargo.toml: 
```toml
embassy-stm32-plus = { version = "0.1.2", features = ["stm32f103rc"] }
embassy-executor = { version = "0.6.0", features = ["arch-cortex-m", "executor-thread", "defmt", "integrated-timers"] }
defmt-rtt = "0.4.1"
cortex-m-rt = "0.7.3"
panic-probe = { version = "0.3.2", features = ["print-defmt"] }
```

```rust
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32_plus::embassy_stm32;
use embassy_stm32_plus::embassy_time::Timer;
use embassy_stm32_plus::traits::gpio::output::GpioOutput;
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

</details>

<details>
<summary>usb example</summary>

Cargo.toml file:

```toml
embassy-stm32-plus = { version = "0.1.2", features = ["stm32f103rc"] }
embassy-executor = { version = "0.6.0", features = ["arch-cortex-m", "executor-thread", "defmt", "integrated-timers"] }
embassy-futures = { version = "0.1.1" }
defmt = "0.3.8"
defmt-rtt = "0.4.1"
cortex-m = { version = "0.7.7", features = ["inline-asm", "critical-section-single-core"] }
cortex-m-rt = "0.7.3"
panic-probe = { version = "0.3.2", features = ["print-defmt"] }
```

main.rs file:

```rust
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32_plus::embassy_stm32;
use embassy_stm32_plus::embassy_stm32::peripherals::USB;
use embassy_stm32_plus::embassy_stm32::usb::Driver;
use embassy_stm32_plus::embassy_usb::class::cdc_acm::CdcAcmClass;
use embassy_stm32_plus::embassy_usb::Config;
use embassy_stm32_plus::embassy_usb::driver::EndpointError;
use embassy_stm32_plus::traits::usb::acm_state::AcmState;
use embassy_stm32_plus::traits::usb::buf::UsbBuf;
use embassy_stm32_plus::traits::usb::UsbTrait;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // rcc setting or etc., more see https://github.com/embassy-rs/embassy/blob/main/examples/stm32f3/src/bin/usb_serial.rs
    let p = embassy_stm32::init(Default::default());

    // build default usb device
    let mut usb_buf = UsbBuf::default();
    let mut acm_state = AcmState::default();
    let (mut class, mut usb) = p.USB.build_cdc_acm(p.PA12, p.PA11, &mut usb_buf, &mut acm_state, Config::new(0xc0de, 0xcafe));

    // usb business
    let echo_fut = async {
        loop {
            class.wait_connection().await;
            defmt::info!("Connected");
            let _ = echo(&mut class).await;
            defmt::info!("Disconnected");
        }
    };

    // wait usb business
    embassy_futures::join::join(echo_fut, usb.run()).await;
}

async fn echo<'d>(class: &mut CdcAcmClass<'d, Driver<'d, USB>>) -> Result<(), EndpointError> {
    let mut buf = [0; 64];
    loop {
        let n = class.read_packet(&mut buf).await?;
        let data = &buf[..n];
        defmt::info!("data: {:x}", data);
        class.write_packet(data).await?;
    }
}

```

</details>

### Other instructions

- build linrary(.bin): `cargo objcopy --release -- -O binary app.bin`
- build ihex(.hex): `cargo objcopy --release -- -O ihex app.hex`
- debug see [probe-rs](https://probe.rs/)
- more see [embassy](https://github.com/embassy-rs/embassy)

