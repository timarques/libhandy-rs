# libhandy-rs
This repository contains the rust bindings for libhandy.

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
./regen.sh
```
