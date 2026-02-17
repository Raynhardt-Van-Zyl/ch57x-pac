# ch57x-pac

[![Crates.io](https://img.shields.io/crates/v/ch57x-pac.svg)](https://crates.io/crates/ch57x-pac)
[![docs.rs](https://docs.rs/ch57x-pac/badge.svg)](https://docs.rs/ch57x-pac)
[![License: MIT OR Apache-2.0](https://img.shields.io/crates/l/ch57x-pac.svg)](https://crates.io/crates/ch57x-pac)
[![CI](https://github.com/Raynhardt-Van-Zyl/ch57x-pac/actions/workflows/ci.yml/badge.svg)](https://github.com/Raynhardt-Van-Zyl/ch57x-pac/actions/workflows/ci.yml)

Peripheral Access Crate (PAC) for WCH CH57x RISC-V BLE microcontrollers.

This crate is generated using `svd2rust` and provides register-level access to CH57x peripherals.

## Scope

- `no_std` PAC for CH57x-family register blocks
- Interrupt definitions and vector table support via `rt` feature
- Optional `critical-section` integration for embedded Rust ecosystem crates

## Cargo

```toml
[dependencies]
ch57x-pac = "0.1.3"
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

## CI/CD

- CI runs `cargo check` and `cargo doc` on every push and pull request.
- Publish workflow is configured to publish from version tags (or manual dispatch).
  The publish job requires repository secret `CRATES_IO_TOKEN`.

## License

Licensed under either:

- Apache License, Version 2.0
- MIT license
