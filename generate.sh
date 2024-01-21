#!/usr/bin/env bash

git submodule update --init --recursive

./cmsis-svd-generator/generate_svd.py -d jh7110.dts -o jh7110.svd

mkdir -p out

svd2rust -i jh7110.svd --target=riscv -o out
form -i out/lib.rs -o src

cargo fmt

rm -rf out
