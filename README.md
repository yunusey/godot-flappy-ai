# godot-flappy-ai
AI learns to play Flappy Bird using Genetic Algorithms.

## Exporting to Linux, Mac, and Windows
For ordinary builds, we just need to run
```bash
cd ./rust/godot-neural-networks/ && cargo build
```
I didn't test with Mac or Windows, but it should be fine. Then, we need to copy the resulting `target/debug/libgodot_neural_networks.so` to the `game/extern` directory. This is 

## Exporting to Web
It is a bit more complex. Here's how I did it and you can, hopefully, reproduce if you are a Nix user.

1. First, you will need to go to the `rust/godot-neural-networks` directory.

2. If you are using `direnv`, you are going to end up in a shell with nightly rust installed along with `emscripten` which is needed to link to `wasm32-unknown-unknown`. If you do not have a nightly rust, you can follow the instructions in [rustup.rs](https://rustup.rs).

3. Now, we need to build the wasm. If you use Nix, and therefore have the nightly rust, you can just run
```bash
cargo build -Zbuild-std --target wasm32-unknown-emscripten
```

If you are not using Nix, but have the nightly build, you should run
```bash
rustup target add wasm32-unknown-emscripten
rustup toolchain install nightly
rustup component add rust-src --toolchain nightly
rustup target add wasm32-unknown-emscripten --toolchain nightly
cargo +nightly build -Zbuild-std --target wasm32-unknown-emscripten
```
