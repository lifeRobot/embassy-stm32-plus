pub fn main() {
    // ignore cfg check
    let ignore = vec![
        "PB8", "PB9", "PC10", "PC11",
        "PD3", "PD4", "PD5", "PD6", "PD8", "PD9", "PD10", "PD11", "PD12",
        "ADC1", "ADC2", "ADC3", "DAC", "DAC1",
        "CAN", "CAN1", "CAN2", "CAN_PD0", "CAN_PD1", "CAN1_PD0", "CAN1_PD1", "CRC",
        "ETH", "I2C1", "I2C2", "SPI1", "SPI2", "SPI3",
        "USART1", "USART2", "USART3", "UART4", "UART5"]
        .join(",");
    println!("cargo:rustc-check-cfg=cfg({ignore})");

    /*let cfg_list = stm32_metapac::metadata::METADATA.peripherals.iter().map(|p|{p.name}).collect::<Vec<&str>>().join(",");
    println!("\n\n\n\n\n{cfg_list}\n\n\n\n\n");*/

    // add stm32 peripheral to cfg
    let mut has_cfg = Vec::with_capacity(1);
    for p in stm32_metapac::metadata::METADATA.peripherals.iter() {
        add_cfg(&mut has_cfg, p.name);
        for pin in p.pins.iter() {
            add_cfg(&mut has_cfg, pin.pin);
            add_cfg(&mut has_cfg, format!("{}_{}", p.name, pin.pin));
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
