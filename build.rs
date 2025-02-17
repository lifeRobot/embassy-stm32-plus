pub fn main() {
    // ignore cfg check
    let ignore = vec![
        "STM32C0", "STM32F1", "STM32F102", "STM32F103",
        "PA9", "PA10", "PB8", "PB9", "PC10", "PC11",
        "PD3", "PD4", "PD5", "PD6", "PD8", "PD9", "PD10", "PD11", "PD12",
        "ADC1", "ADC2", "ADC3", "DAC", "DAC1",
        "CAN", "CAN1", "CAN2", "CAN_PD0", "CAN_PD1", "CAN1_PD0", "CAN1_PD1",
        "CRC", "CRC_v3",
        "ETH", "I2C1", "I2C1_PA9", "I2C1_PA10", "I2C1_PC14", "I2C1_SCL_PB7",
        "I2C2", "SPI1", "SPI1_PA1", "SPI1_PA2", "SPI1_PA5", "SPI1_PA6", "SPI1_PA7", "SPI1_PA11", "SPI1_PA12",
        "SPI1_PB3", "SPI1_PB4", "SPI1_PB5", "SPI1_PB6",
        "SPI2", "SPI3", "SPI3_PC10", "SPI3_PC11", "SPI3_PC12",
        "USART1", "USART1_PA0", "USART1_PA1", "USART1_PA8", "USART1_PA11", "USART1_PA12", "USART1_PA14", "USART1_PA15",
        "USART1_PB2", "USART1_PB3", "USART1_PB4", "USART1_PB6", "USART1_PC14",
        "USART1_RX_PA8", "USART1_CTS_PB6",
        "USART2", "USART2_PA3", "USART2_PA5", "USART2_PA8", "USART2_PA13", "USART2_PA14", "USART2_PA15",
        "USART2_PB7", "USART2_PB8", "USART2_PB9", "USART2_PC14", "USART2_PD3", "USART2_PD4", "USART2_PD5", "USART2_PD6",
        "USART2_TX_PA4", "USART3", "UART4", "UART5", "USB", "USB_OTG_FS"]
        .join(",");
    println!("cargo:rustc-check-cfg=cfg({ignore})");

    /*let cfg_list = stm32_metapac::metadata::METADATA.peripherals.iter().map(|p|{p.name}).collect::<Vec<&str>>().join(",");
    println!("\n\n\n\n\n{cfg_list}\n\n\n\n\n");*/


    // add stm32 peripheral to cfg
    let mut has_cfg = Vec::with_capacity(1);
    add_cfg(&mut has_cfg, stm32_metapac::metadata::METADATA.family);
    add_cfg(&mut has_cfg, &stm32_metapac::metadata::METADATA.name[0..9]);
    for p in stm32_metapac::metadata::METADATA.peripherals.iter() {
        add_cfg(&mut has_cfg, p.name);
        for pin in p.pins.iter() {
            add_cfg(&mut has_cfg, pin.pin);
            add_cfg(&mut has_cfg, format!("{}_{}", p.name, pin.pin));
            add_cfg(&mut has_cfg, format!("{}_{}_{}", p.name, pin.signal, pin.pin));
        }
        if let Some(r) = p.registers.as_ref() {
            add_cfg(&mut has_cfg, format!("{}_{}", p.name, r.version));
        }
    }
}

/// add cfg
fn add_cfg(has_cfg: &mut Vec<String>, cfg: impl Into<String>) {
    let cfg = cfg.into();
    // if has cfg, return
    if has_cfg.contains(&cfg) { return; }

    // println!("cargo:rustc-check-cfg=cfg({cfg})");
    println!("cargo:rustc-cfg={cfg}");
    // not cfg, add to cfg
    has_cfg.push(cfg);
}
