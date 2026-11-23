# bps — bypassOS Package Manager

A lightweight, fast package manager written in Rust for **bypassOS**.

> **⚠️ Status: MVP / Early Development**

##  Features (Current MVP)

- Install packages
- Update bypassOS
- Written in pure Rust for memory safety

##  Building from Source

1. Clone the repository:
   ```bash
   git clone https://github.com/bypassOS-dev/bypassOS/
   cd bypassOS/bps
   ```

2. Build the release binary:
   ```bash
   cargo build --release
   ```

3. Move the binary to your `$PATH`:
   ```bash
   sudo cp target/release/bps /usr/local/bin/
   ```

##  Usage

```bash
# Install a package
bps install <package>

# Update your system
bps update
```

## Contributing

Contributions, bug reports, and feature requests are welcome! Since `bps` is an early-stage project for bypassOS, feel free to open an issue to discuss major changes before submitting a PR.

<div align="center">

#  ⚠️ ⚠️ ⚠️  Source code is available  [HERE](https://github.com/bypassOS-dev/bypassOS/tree/main/bps)⚠️ ⚠️ ⚠️ 

</div>




