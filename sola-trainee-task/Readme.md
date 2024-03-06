Author: Timotej Ponek, timotej.ponek@gmail.com

# About
Program that counts how many viruses are there in the world, as specified by the Solana Auditor Entry Task

## Rust version used
Program was compiled with version `rustc 1.68.0 (2c8cc3432 2023-03-06)` of rust compiler. Compiling with newer rust versions should be ok, but using older versions can result in compilation failure.

## How to run
# TODO how to install clap?
To build, run:
```cargo build --release```

To run, run:
```cargo run --release```

`--release` argument is needed in order to process big numbers like 10^10 in reasonable time (otherwise you run program in debug mode)

As first line of the input, program expects number of questions to be answered (`1<=T<=1000`). Following T lines are questions (numbers in range `4<=X<=10^10`) about how many viruses there are in the world, if there were X clicks/replications. Program calculates answers only after all questions have been submitted.

You can run program with `cargo run` and provide all necessary inputs, or provide file to be read from as an argument `cargo run --release -- --file <PATH-TO-FILE>`.

Program expects that you input valid integer numbers, otherwise it will fail. It will