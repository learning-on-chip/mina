# Mina

Mina is a tool for generating traffic data.

## Installation

Install Rust as described [here](https://www.rust-lang.org/tools/install):

```sh
curl https://sh.rustup.rs -sSf | sh
```

Install the tool:

```sh
cargo install --git https://github.com/learning-on-chip/mina
```

## Usage

The tool has the following options:

```
Usage: mina [options]

Options:
    --input <string>    File for reading a sequence of arrival times
    --output <string>   File for writing a sequence of arrival times
    --length <number>   Number of arrival times to be generated
    --seed <number>     Seed for the random number generator
    --help              Flag for showing this help message
```

For instance:

```sh
mina --input tests/fixtures/data.csv --output data.csv --length 10000 --seed 42
```
