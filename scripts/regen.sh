gen_base="jh7110"

git submodule update --init --recursive

../cmsis-svd-generator/generate_svd.py -d ${gen_base}.dts -o ${gen_base}.svd

mkdir -p out

svd2rust -i ${gen_base}.svd --target=riscv -o out
form -i out/lib.rs -o src

cargo fmt

rm -rf out
