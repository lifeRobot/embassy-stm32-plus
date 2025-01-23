pub fn main() {
    // ignore cfg check
    println!("cargo:rustc-check-cfg=cfg(PC10,PC11,PD3,PD4,PD5,PD6,PD8,PD9,PD11,PD12,ADC1,ADC2,CAN,CAN1,CAN2,CAN_PD0,CAN_PD1,CAN1_PD0,CAN1_PD1,USART1,USART2,USART3,UART4,UART5)");

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
