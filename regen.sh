#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail


CRATE_API=libhandy
CRATE_SYS=libhandy-sys

CONFIG_API=${CRATE_API}/Gir.toml
CONFIG_SYS=${CRATE_SYS}/gir-libhandy.toml

GIR=./target/release/gir

# Build the gir tool
git submodule update --init
(cd gir && cargo build --release)

# Regenerate the sys crate
rm -rf ${CRATE_SYS}/{build.rs, src/auto}
${GIR} -c ${CONFIG_SYS} -d gir-files -o ${CRATE_SYS}

# Regenerate the api crate
rm -rf ${CRATE_API}/src/auto
${GIR} -c ${CONFIG_API} -d gir-files -o ${CRATE_API}

