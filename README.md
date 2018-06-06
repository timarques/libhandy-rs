# gspell-sys-rs
To build the gspell rust bindings, you need to build the sys and api parts
```shell
git clone https://github.com/gtk-rs/gir.git
cd gir
git clone https://github.com/gtk-rs/gir-files
git clone https://gitlab.gnome.org/jsparber/gspell-sys-rs.git
cp gspell-sys-rs/Gspell-1.0.gir gir-files
cp gspell-sys-rs/gir-gspell.toml ./
cargo run --release -- -c gir-gspell.toml -d gir-files -m sys -o gspell-sys
git clone https://gitlab.gnome.org/jsparber/gspell-rs.git
cp gspell-rs/Gir.toml ./
cargo run --release -- -c Gir.toml -d gir-files -o gspell
cp gspell-rs/src/lib.rs gspell-rs/src/manual.rs gspell-rs/src/rt.rs gspell/src/
cp gspell-rs/Cargo.toml gspell
```
https://github.com/gtk-rs/gir
