# rust-utils
A small Rust utility crate

## What is rust_utils?
Put simply, it's a small utility crate with some functions I like reusing across my Rust projects.
There's not much in here. I just kind of add stuff whevever I write a function and decide that
it's generic enough to be reusable across projects.

## How to use
If you want to use functionality from this crate in your own Rust projects, add it to
your `Cargo.toml`:
```toml
[dependencies]
rust_utils = { git = "https://github.com/itzGalactica/rust-utils.git" }
```

You can then use its functionality inside your code, for example:
```rust
use rust_utils;

fn main() {
  let args = rust_utils::get_argv();
  dbg!(&args);
}
```
