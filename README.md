# libhandy-rs
This repository contains the WIP rust bindings for libhandy.

## Using
Add this line to your Cargo file
```
[dependencies]
libhandy = { git = "https://gitlab.gnome.org/jsparber/libhandy-rs" }
```

## Build
We use [gir](https://github.com/gtk-rs/gir) to generate rust libhandy bindings. The bindings are split in two parts, sys and api.
```shell
git clone https://github.com/gtk-rs/gir.git
cd gir
git clone https://github.com/gtk-rs/gir-files
git clone https://gitlab.gnome.org/jsparber/libhandy-sys-rs.git
cp libhandy-sys-rs/Gspell-1.0.gir gir-files
cp libhandy-sys-rs/gir-libhandy.toml ./
cargo run --release -- -c gir-libhandy.toml -d gir-files -m sys -o libhandy-sys
git clone https://gitlab.gnome.org/jsparber/libhandy-rs.git
cp libhandy-rs/Gir.toml ./
cargo run --release -- -c Gir.toml -d gir-files -o libhandy
cp libhandy-rs/src/lib.rs libhandy-rs/src/manual.rs libhandy-rs/src/rt.rs libhandy/src/
cp libhandy-rs/Cargo.toml libhandy
```


