# A Hello World for Hexagon

Rust nightly and [xargo](https://github.com/japaric/xargo) should be installed first.

A copy of rust is necessary to build libcore -

```
% git clone https://github.com/rust-lang/rust
```

Check what git commit your rust is based on -

```
% rustc --version
rustc 1.19.0-dev (ac254fbe7 2017-05-15)
```

And set your rust repo to that commit -
```
% cd rust
% git checkout ac254fbe7
% cd ..
```

Xargo needs to know where the rust source is -
```
% export XARGO_RUST_SRC=$PWD/rust/src
```

`RUST_TARGET_PATH` needs to be set to work around a bug that prevents us from specifying the json target file directly. This points to the directory where `hexagon-unknown-elf.json` is stored, which is the top of this repo.

```
% export RUST_TARGET_PATH=$PWD
```

Make sure the hexagon tools are in the `PATH`.
```
% export PATH="$HEXAGON_SDK_ROOT/tools/HEXAGON_Tools/8.0.08/Tools/bin:$PATH"
```

Build
```
xargo build --target hexagon-unknown-elf --release
```

Run
```
hexagon-sim target/hexagon-unknown-elf/release/hexagon-hello-world
```
