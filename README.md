[![Build Status](https://travis-ci.com/jaredforth/clean_url.svg?branch=master)](https://travis-ci.com/jaredforth/clean_url)
[![Build status](https://ci.appveyor.com/api/projects/status/ijwnxxl37ivyy67f?svg=true)](https://ci.appveyor.com/project/jaredforth/clean-url)
[![Crate](https://img.shields.io/crates/v/clean_url.svg)](https://crates.io/crates/clean_url)
[![API](https://docs.rs/clean_url/badge.svg)](https://docs.rs/clean_url)
![Crates.io](https://img.shields.io/crates/l/clean_url)
![Crates.io](https://img.shields.io/crates/v/clean_url)
![GitHub top language](https://img.shields.io/github/languages/top/jaredforth/clean_url)
![Crates.io](https://img.shields.io/crates/d/clean_url)

# clean_url

## About

This crate takes a URL, formats it correctly, and returns the version of the URL that returns a 200 series response. 

## Table of Contents

- [clean_url](#clean_url)
  - [About](#about)
  - [Table of Contents](#table-of-contents)
  - [Installation and Usage](#installation-and-usage)
    - [Installation](#installation)
    - [Usage](#usage)
  - [License](#license)


Documentation:
-   [API Reference](https://docs.rs/clean_url)

## Installation and Usage

### Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
clean_url = "0.1"
```

### Usage

```rust
async fn main() {
    let url = return_url(String::from("example.com")).await;
    assert_eq(Some("http://example.com"), url);
}
```

## License

**clean_url** is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT), and
[COPYRIGHT](COPYRIGHT) for details.
