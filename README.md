# gif-maker
Command CLI app developed with Rust

## Cargo Commands

cargo run -- make-gif --open-file-directory test-images/numbers --save-gif-file-path test-output/test.gif


Custom 
```
cargo run -- make-custom --open-file-directory test-images/numbers --save-gif-file-path test-output/custom01.gif --width 100 --height 100 --colour-map "0x32, 0x32, 0x32, 0x32, 0x32, 0x32"
```