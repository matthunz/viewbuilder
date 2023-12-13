<div align="center">
<h1>Viewbuilder</h1>
 <a href="https://crates.io/crates/viewbuilder">
    <img src="https://img.shields.io/crates/v/viewbuilder?style=flat-square"
    alt="Crates.io version" />
  </a>
  <a href="https://docs.rs/viewbuilder/latest/viewbuilder/">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
   <a href="https://github.com/matthunz/viewbuilder/actions">
    <img src="https://github.com/matthunz/viewbuilder/actions/workflows/ci.yml/badge.svg"
      alt="CI status" />
  </a>
</div>

<div align="center">
 <a href="https://github.com/matthunz/viewbuilder/tree/main/examples">Examples</a>
</div>

<br>

A cross-platform user interface framework for Rust.

Viewbuilder is a moduler GUI library that can be used as an entire framework, or with individual parts.

```rust
use concoct::{Handler, Object};
use viewbuilder::{window, UserInterface, Window};

struct App;

impl Object for App {}

impl Handler<window::Resized> for App {
    fn handle(&mut self, _cx: concoct::Context<Self>, msg: window::Resized) {
        dbg!(msg);
    }
}

fn main() {
    let ui = UserInterface::default();
    let _guard = ui.enter();

    let window = Window::default().spawn();

    let app = App.spawn();
    window.bind(&app);

    ui.run()
}
```

## Getting started

Instatllation is simple with:

```sh
cargo add viewbuilder --features full
```
