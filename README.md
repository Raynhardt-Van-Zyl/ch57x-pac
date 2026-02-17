# ch57x-pac

Peripheral Access Crate (PAC) for WCH CH57x RISC-V BLE microcontrollers.

This crate is generated using `svd2rust` and provides register-level access to CH57x peripherals.

## Scope

- `no_std` PAC for CH57x-family register blocks
- Interrupt definitions and vector table support via `rt` feature
- Optional `critical-section` integration for embedded Rust ecosystem crates

## Cargo

```toml
[dependencies]
ch57x-pac = "0.1.2"
```

## Features

- `rt` (default): exports interrupt symbols/vector table glue expected by runtime crates
- `critical-section`: enables `critical-section` support

## Minimal Usage

```rust
let p = ch57x_pac::Peripherals::take().unwrap();
p.sys.r8_clk_sys_cfg().modify(|_, w| unsafe { w.bits(0) });
```

## docs.rs Notes

docs.rs builds with:

- target: `riscv32imac-unknown-none-elf`
- features: `rt`, `critical-section`

## License

Licensed under either:

- Apache License, Version 2.0
- MIT license
