name := "bevy-snake"
out_dir := "out"

test:
	echo {{name}}_bg

# we run this with -f so that we dont get any errors
# -r is for recursion of course
clear-web:
	rm -rf {{out_dir}}

build-web: clear-web
	# build wasm file in release mode
	cargo build --release --target wasm32-unknown-unknown

	# generate proper javascript to /out and optimize banary
	wasm-bindgen --no-typescript --target web --out-dir {{out_dir}} target/wasm32-unknown-unknown/release/{{name}}.wasm
	wasm-opt {{out_dir}}/{{name}}_bg.wasm -Os -o {{out_dir}}/{{name}}.wasm 

	# swap unoptimized binary with the optimized one
	rm {{out_dir}}/{{name}}_bg.wasm
	mv {{out_dir}}/{{name}}.wasm {{out_dir}}/{{name}}_bg.wasm