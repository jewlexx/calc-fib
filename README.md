# Fibonacci Like

A small crate to help you interact with second order sequences, such as the Fibonacci sequence, with no_std support.

## Features

- Find the position of a given number
- Find the number at a given position
- Custom second order sequences

## Installation

```toml
[dependencies]
fibonacci_like = "0.1"
```

## Usage

```rust
use fibonacci_like::IntoNumber;

let number = fibonacci_like::Sequence::fibonacci().find(69.into_number());

println!("The \"{nth}\" number of the fibonacci sequence is:\n{number}");
```

## License

One of the following, at your choice:

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: APACHE](https://img.shields.io/badge/License-APACHE-green.svg)](https://opensource.org/licenses/APACHE)

**Made with 💗 by [Juliette Cordor](https://github.com/jewlexx)**
