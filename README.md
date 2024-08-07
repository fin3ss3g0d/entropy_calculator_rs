# Entropy Calculator

![thanos-rust](images/thanos-rust.jpg)

This repository contains a Rust application that uses Shannon's Entropy formula to calculate entropy for all strings in a given file and logs strings over a certain threshold to an output file. The program expects an input file with one string per line.

## Building

```
rustup target add x86_64-pc-windows-msvc
rustup target add x86_64-unknown-linux-gnu
cargo build --target x86_64-pc-windows-msvc --release
cargo/cross build --target x86_64-unknown-linux-gnu --release
```

## Usage

Depending on your use case, you may want to test your data set to see the types of entropy levels within it so I have included a `--test` flag that will log each entropy calculation along with its corresponding string to an output file. Once you have tested, you can supply the program with the entropy threshold you desire and it will only log strings above this threshold.

```
entropy_calculator.exe <input_file_path> <output_file_path> [<entropy_threshold> | --test]
```
