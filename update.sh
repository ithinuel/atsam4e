#!/usr/bin/env bash

set -xe

# INSTALL DEPENDENCIES

cargo install svd2rust --git https://github.com/rust-embedded/svd2rust
cargo install form

# PATCH SVD FILES AND GENERATE CRATES

TOP="${PWD}"

for xsl in svd/devices/*\.xsl; do
  chip=$(basename "${xsl}" .xsl)
  CHIP=$(echo "${chip}" | tr '[:lower:]' '[:upper:]')
  svd=svd/${CHIP}.svd

  pushd "${TOP}/pac/${chip}"

  xsltproc "${TOP}/${xsl}" "${TOP}/${svd}" | svd2rust --nightly

  rm -rf src/
  form -i lib.rs -o src
  rm lib.rs
  cargo fmt
  rustfmt build.rs

  popd
done
