crate=$1
gen_base=""

if [[ $crate = "jh7110-vf2-12a-pac" ]]; then
	gen_base="jh7110-starfive-visionfive-2-v1.2a"
elif [[ $crate = "jh7110-vf2-13b-pac" ]]; then
	gen_base="jh7110-starfive-visionfive-2-v1.3b"
else
	echo "Invalid crate, must be one of: [jh7110-vf2-12a-pac, jh7110-vf2-13b-pac]"
fi

git submodule update --init --recursive

pushd $crate

../cmsis-svd-generator/generate_svd.py -d ${gen_base}.dts -o ${gen_base}.svd

mkdir -p out

svd2rust -i ${gen_base}.svd --target=riscv -o out
form -i out/lib.rs -o src

cargo fmt

rm -rf out

popd
