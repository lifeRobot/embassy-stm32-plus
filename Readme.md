### goal

Make Rust Embedded simpler

embassy-stm32_plus is a library based on [embassy-stm32](https://crates.io/crates/embassy-stm32) secondary encapsulation, 
which supports generating peripheral objects directly through Device stream without manually setting Irqs interrupts, 
such as directly generating gpio output objects `p.PA8.output()` and directly generating 
UART objects    
```
p.USART1.builder(Uart1Tx::PA9(p.PA9), Uart1Rx::PA10(p.PA10))
    .build(p.DMA1_CH4, p.DMA1_CH5);
```   
etc

### support now

- STM32F1 &#10004;
- STM32C0 &#10004;
- more support comming soon

### example

build.rs file:   
(build.rs is necessary, otherwise it may result in inability to burn)
```rust
fn main() {
    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");

    // Set DEFMT_LOG=info through environment variables to enable probe rs to support defmt: Output of info level log for info
    println!("cargo:rustc-env=DEFMT_LOG=info");
}
```

<details open>
<summary>uart example</summary>

Cargo.toml file :

```toml
embassy-stm32-plus = { version = "0.2.1", features = ["stm32f103rc", "exti"] }
embassy-executor = { version = "0.7.0", features = ["arch-cortex-m", "executor-thread"] }

cortex-m-rt = "0.7.5"
defmt = "0.3.10"
```

main.rs file :

```rust
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32_plus::builder::uart::uart1::rx::Uart1Rx;
use embassy_stm32_plus::builder::uart::uart1::tx::Uart1Tx;
use embassy_stm32_plus::embassy_stm32;
use embassy_stm32_plus::embassy_stm32::mode::Async;
use embassy_stm32_plus::embassy_stm32::usart::{Error, Uart};
use embassy_stm32_plus::traits::uart::uart1::Uart1Trait;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // init stm32, get Peripheral
    let p = embassy_stm32::init(Default::default());

    // set uart baud rate etc
    // let mut uart_config = embassy_stm32::usart::Config::default();
    // uart_config.baudrate = 9600;

    let mut uart = p.USART1.builder(Uart1Tx::PA9(p.PA9), Uart1Rx::PA10(p.PA10))
        // set uart config
        // .config(Config::default())
        .build(p.DMA1_CH4, p.DMA1_CH5).unwrap();
    defmt::info!("uart initialized!");

    let mut buf = [0u8; 1024];
    loop {
        // wait uart read
        let len = match uart.read_until_idle(&mut buf).await {
            Ok(len) => { len }
            Err(e) => {
                defmt::error!("uart read error: {:?}", e);
                continue;
            }
        };

        defmt::info!("uart read, len is {}", len);

        // reply uart
        if let Err(e) = reply_write_flush(&mut uart, &buf[0..len]).await {
            defmt::error!("uart write error: {:?}",e);
        }
    }
}

/// uart write and flush
async fn reply_write_flush(uart: &mut Uart<'static, Async>, buf: &[u8]) -> Result<(), Error> {
    uart.write(b"copy, ").await?;
    uart.write(buf).await?;
    uart.flush().await
}

```

</details>

<details>
<summary>gpio example</summary>

Cargo.toml:

```toml
embassy-stm32-plus = { version = "0.2.1", features = ["stm32f103rc", "exti"] }
embassy-executor = { version = "0.7.0", features = ["arch-cortex-m", "executor-thread"] }
embassy-time = "0.4.0"

cortex-m-rt = "0.7.5"
defmt = "0.3.10"
```

```rust
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32_plus::embassy_stm32;
use embassy_stm32_plus::traits::gpio::GpioTrait;
use embassy_time::Timer;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // init stm32, get Peripheral
    let p = embassy_stm32::init(Default::default());

    // simple get output/input gpio
    let mut led = p.PA8.output();
    defmt::info!("led initialized!");

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
<summary>more example</summary>

more example coming soon,   
you can try using the following method to directly generate peripheral protocol objects:
`p.PA8.input()`   
`p.PA8.output()`   
`p.ADC.build()`   
`p.CAN1.build(tx,rx)`   
`p.CRC.build()`   
`p.DAC1.builder(ch1,ch2).build(dma_ch1,dma_ch2)`   
`p.ETH.builder(pins,phy).build.`   
`p.FLASH.build()`   
`p.I2C1.builder(scl,sda).build(tx_dma,rx_dma)`   
`p.SPI1.builder(sck,mosi,miso).build(tx_dma,rx_dma)`   
`p.UART1.builder(tx,rx).build(tx_dma,rx_dma)`   
`p.UID.uid()`   
`p.USB.builder(dp,dm).build_cdc_acm(config,usb_buf,state)`   
`p.USB_OTG_FS.builder(dp,dm).build_cdc_acm(config,ep_buf,usb_buf,state)`   
`p.IWDG.build(timeout_us)`

for more API interfaces, please refer to [docs.rs](https://docs.rs/embassy-stm32-plus/0.2.1/embassy_stm32_plus/)

</details>


### Other instructions

- build linrary(.bin): `cargo objcopy --release -- -O binary app.bin`
- build ihex(.hex): `cargo objcopy --release -- -O ihex app.hex`
- debug see [probe-rs](https://probe.rs/)
- more see [embassy](https://github.com/embassy-rs/embassy)

