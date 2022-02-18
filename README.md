# brr64

Find base64 encoded content without decoding at all. 

### Usage

There are two usages:

  - A [brr64 web-version](https://ellcs.github.io/brr64/) allows you to
    generate regular expressions and search with common tools like `grep`.
  - A command line interface (CLI) named `brr64`.

### Install and build

You already set up your `rustup` and build `brr64` the following way:

    rustup override set nightly
    cargo build --release

After that you can run the release version:

    ./target/release/brr64 "CTF{" /dev/urandom
