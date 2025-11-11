# ch57x-pac

Peripheral Access Crate for the WCH CH57x series microcontrollers.

This crate provides a safe Rust API for accessing the peripherals of the WCH CH57x RISC-V microcontrollers. It is generated from the SVD file using `svd2rust`.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
ch57x-pac = { git = "https://github.com/Raynhardt-Van-Zyl/ch57x-pac" }
```

Or for local development:

```toml
[dependencies]
ch57x-pac = { path = "../ch57x-pac" }
```

Then, in your code:

```rust
use ch57x_pac::UART0;
```

## Building

To build the crate:

```bash
cargo build --target riscv32i-unknown-none-elf
```

## License

Licensed under MIT OR Apache-2.0.