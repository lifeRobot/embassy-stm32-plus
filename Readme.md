### goal

Make Rust Embedded simpler

### support now

- stm32f100xx &#10004;
- stm32f101xx &#10004;
- stm32f102xx &#10004;
- stm32f103xx &#10004;
- stm32f105xx &#10004;
- stm32f107xx &#10004;
- more support comming soon

### example

<details open>
<summary>uart example</summary>

Cargo.toml file :

```toml
embassy-stm32-plus = { version = "0.1.4", features = ["stm32f103rc"] }
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
embassy-stm32-plus = { version = "0.1.4", features = ["stm32f103rc"] }
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
embassy-stm32-plus = { version = "0.1.4", features = ["stm32f103rc"] }
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

<details>
<summary>usb otg example</summary>

Cargo.toml file:

```toml
embassy-stm32-plus = { version = "0.1.4", features = ["stm32f105vc"] }
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
use embassy_stm32_plus::embassy_stm32::peripherals::USB_OTG_FS;
use embassy_stm32_plus::embassy_stm32::usb_otg::Driver;
use embassy_stm32_plus::embassy_usb::class::cdc_acm::CdcAcmClass;
use embassy_stm32_plus::embassy_usb::Config;
use embassy_stm32_plus::embassy_usb::driver::EndpointError;
use embassy_stm32_plus::traits::usb::acm_state::AcmState;
use embassy_stm32_plus::traits::usb::buf::UsbBuf;
use embassy_stm32_plus::traits::usb::otg::UsbOtgTrait;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // rcc setting or etc., more see https://github.com/embassy-rs/embassy/blob/main/examples/stm32f4/src/bin/usb_serial.rs
    let p = embassy_stm32::init(Default::default());

    // build default usb device
    let mut buffer = [0; 256];
    let mut usb_buf = UsbBuf::default();
    let mut state = AcmState::default();
    let (mut class, mut usb) = p.USB_OTG_FS.build_cdc_acm(p.PA12, p.PA11, &mut buffer, &mut usb_buf, &mut state, Config::new(0xc0de, 0xcafe));

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

async fn echo<'d>(class: &mut CdcAcmClass<'d, Driver<'d, USB_OTG_FS>>) -> Result<(), EndpointError> {
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

<details>
<summary>eth example (stm32f107xx)</summary>
Tips: currently, only stm32f107xx in the cargo crate supports eth

Cargo.toml file:

```toml
embassy-stm32-plus = { version = "0.1.4", features = ["stm32f107vc"] }
embassy-executor = { version = "0.6.0", features = ["arch-cortex-m", "executor-thread", "defmt", "integrated-timers"] }
embassy-futures = { version = "0.1.1" }
embassy-net = { version = "0.4.0", features = ["dhcpv4", "tcp"] }
embedded-io-async = "0.6.1"
static_cell = "2.1.0"
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
use embassy_net::{Ipv4Address, Stack, StackResources};
use embassy_net::tcp::TcpSocket;
use embassy_stm32_plus::{embassy_stm32, embassy_time};
use embassy_stm32_plus::embassy_stm32::eth::generic_smi::GenericSMI;
use embassy_stm32_plus::embassy_stm32::eth::{Ethernet, PacketQueue};
use embassy_stm32_plus::embassy_stm32::peripherals::ETH;
use embassy_stm32_plus::embassy_time::Timer;
use embassy_stm32_plus::traits::eth::Eth1;
use embedded_io_async::Write;
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // rcc setting or etc., more see https://github.com/embassy-rs/embassy/blob/main/examples/stm32f4/src/bin/eth.rs
    let p = embassy_stm32::init(Default::default());

    // simple build eth
    static PACKETS: StaticCell<PacketQueue<4, 4>> = StaticCell::new();
    let mac_addr = [0x00, 0x00, 0xDE, 0xAD, 0xBE, 0xEF];
    let eth = p.ETH.eth1(&p, PACKETS.init(PacketQueue::<4, 4>::new()), GenericSMI::new(0), mac_addr);

    // Init network stack, copy from https://github.com/embassy-rs/embassy/blob/main/examples/stm32f4/src/bin/eth.rs
    let config = embassy_net::Config::dhcpv4(Default::default());
    static STACK: StaticCell<Stack<Ethernet<ETH, GenericSMI>>> = StaticCell::new();
    static RESOURCES: StaticCell<StackResources<3>> = StaticCell::new();
    // stm32f107 not support rng, random seed set default 0
    let stack = &*STACK.init(Stack::new(eth, config, RESOURCES.init(StackResources::new()), 0));
    defmt::unwrap!(spawner.spawn(net_task(stack)));
    stack.wait_config_up().await;

    defmt::info!("Network task initialized");
    // Then we can use it!
    let mut rx_buffer = [0; 4096];
    let mut tx_buffer = [0; 4096];

    loop {
        let mut socket = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);

        socket.set_timeout(Some(embassy_time::Duration::from_secs(10)));

        let remote_endpoint = (Ipv4Address::new(10, 42, 0, 1), 8000);
        defmt::info!("connecting...");
        let r = socket.connect(remote_endpoint).await;
        if let Err(e) = r {
            defmt::info!("connect error: {:?}", e);
            Timer::after_secs(1).await;
            continue;
        }
        defmt::info!("connected!");
        let buf = [0; 1024];
        loop {
            let r = socket.write_all(&buf).await;
            if let Err(e) = r {
                defmt::info!("write error: {:?}", e);
                break;
            }
            Timer::after_secs(1).await;
        }
    }
}

#[embassy_executor::task]
async fn net_task(stack: &'static Stack<Ethernet<'_, ETH, GenericSMI>>) -> ! {
    stack.run().await
}
```

</details>

<details>
<summary>eth w5500 example</summary>

1. STM32F1xxxx currently does not support ETH W5500 because STM32F1xxxxx does not support exti. more see:
   https://github.com/embassy-rs/embassy/blob/main/examples/stm32f4/src/bin/eth_w5500.rs
2. This is determined by the limitations of crates [`embassy-stm32`](https://crates.io/crates/embassy-stm32)
   and [`embassy-net-wiznet`](https://crates.io/crates/embassy-net-wiznet). You can try using RTOS to implement support
   for eth w5500, more see [stm32f1](https://crates.io/crates/stm32f1)
   or [rust-embedded](https://github.com/rust-embedded)

</details>

### Other instructions

- build linrary(.bin): `cargo objcopy --release -- -O binary app.bin`
- build ihex(.hex): `cargo objcopy --release -- -O ihex app.hex`
- debug see [probe-rs](https://probe.rs/)
- more see [embassy](https://github.com/embassy-rs/embassy)

