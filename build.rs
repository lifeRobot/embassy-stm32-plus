pub fn main() {
    // add stm32 peripheral to cfg
    let mut has_cfg = Vec::with_capacity(1);
    for p in stm32_metapac::metadata::METADATA.peripherals.iter() {
        add_cfg(&mut has_cfg, p.name);
        for pin in p.pins.iter() {
            add_cfg(&mut has_cfg, pin.pin);
        }
    }
}

/// add cfg
fn add_cfg(has_cfg: &mut Vec<&'static str>, cfg: &'static str) {
    // if has cfg, return
    if has_cfg.contains(&cfg) { return; }

    // not cfg, add to cfg
    has_cfg.push(cfg);
    println!("cargo:rustc-check-cfg=cfg({cfg})");
    println!("cargo:rustc-cfg={cfg}");
}
