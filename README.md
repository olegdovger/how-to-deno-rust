# how-to-deno-rust
This is POC on how to use Rust in Deno

Install Rust and Deno before reading any further steps bellow

How to start:

1) Run `cargo run serve`
2) Modify script lib\math\src\lib.rs
3) Wait for building rust library "lib\math"
4) Run `deno run --allow-read main.ts`


What command `cargo run serve` does?  

1) It watches folders under "lib" folder (like "math" folder)
2) It runs command "wasm-pack build "lib\\__any_lib_folder__" (like "lib\\__math__")

As result we can run WASM scripts using Deno
