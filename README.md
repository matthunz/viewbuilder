<div align="center">
<h1>Viewbuilder</h1>
 <a href="https://crates.io/crates/viewbuilder">
    <img src="https://img.shields.io/crates/v/viewbuilder?style=flat-square"
    alt="Crates.io version" />
  </a>
  <a href="https://concoct-rs.github.io/viewbuilder/viewbuilder/index.html">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
   <a href="https://github.com/concoct-rs/viewbuilder/actions">
    <img src="https://github.com/concoct-rs/viewbuilder/actions/workflows/ci.yml/badge.svg"
      alt="CI status" />
  </a>
</div>

<div align="center">
 <a href="https://github.com/concoct-rs/viewbuilder/tree/main/examples">Examples</a>
</div>

<br>

Cross-platform user interface framework for Rust.

This crate provides a moduler GUI library that can be used as an entire framework, or with individual parts.

```rust
use viewbuilder::{object, Object, Runtime};

#[derive(Default)]
pub struct Counter {
    value: i32,
}

#[object]
impl Counter {
    #[signal]
    fn value_changed(&mut self, value: i32);

    #[slot]
    pub fn set(&mut self, value: i32) {
        dbg!(value);
        self.value = value;
        self.value_changed(value);
    }
}

fn main() {
    let a = Counter::default().spawn();
    let b = Counter::default().spawn();

    a.value_changed().bind(&b, Counter::set);

    a.set(2);

    for _ in 0..3 {
        Runtime::current().run();
        Runtime::current().run();
    }
}
```

## Getting started

Instatllation is simple with:

```sh
cargo add viewbuilder --features full
```
