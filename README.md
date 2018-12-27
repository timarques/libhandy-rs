# libhandy-rs
This repository contains the WIP rust bindings for libhandy.

## Using
Add this line to your Cargo file
```
[dependencies]
libhandy = { git = "https://gitlab.gnome.org/World/Rust/libhandy-rs" }
```

## Build
We use [gir](https://github.com/gtk-rs/gir) to generate rust libhandy bindings. The bindings are split in two parts, sys and api.
```shell
git clone --recurse https://gitlab.gnome.org/World/Rust/libhandy-rs.git
cd libhandy-rs/gir
cp ../Handy-0.0.gir ../gir-files
cargo run --release -- -c ../libhandy-sys/gir-libhandy.toml -d ../gir-files -m sys -o ../libhandy-sys
cargo run --release -- -c ../libhandy/Gir.toml -d ../gir-files -o ../libhandy
```
