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
git clone https://github.com/gtk-rs/gir.git
cd gir
git clone https://github.com/gtk-rs/gir-files
cp ../Handy-0.0.gir gir-files
cargo run --release -- -c ../handy-sys/gir-libhandy.toml -d gir-files -m sys -o ../handy-sys
cargo run --release -- -c ../handy/Gir.toml -d gir-files -o ../handy
```


