# brr64

Find base64 encoded content without decoding at all. It figures out all three possible candidates.

### Usage

You can use it in two ways:

  - A [brr64 web-version](https://ellcs.github.io/brr64/) allows you to
    generate regular expressions and search with common tool `grep`.
  - A command line interface (CLI) named `brr64`.

### Install and build

You already set up `rustup`. Than build `brr64` the following way:

```console
user@host:brr64$ rustup override set nightly
user@host:brr64$ cargo build --release
```

After that you can run the release version:

```console
user@host:brr64$ ./target/release/brr64 "CTF{" /dev/urandom
```
