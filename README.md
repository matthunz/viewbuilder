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
use concoct::{Handle, Object, Slot};
use viewbuilder::native::{window, Window};
use winit::dpi::PhysicalSize;

struct App;

impl Object for App {}

impl Slot<window::Resized> for App {
    fn update(&mut self, _cx: Handle<Self>, msg: window::Resized) {
        dbg!(msg);
    }
}

#[viewbuilder::main]
fn main() {
    let app = App.start();

    let window_a = Window::builder().title("Window A").build().start();
    window_a.bind(&app);

    let window_b = Window::builder().title("Window B").build().start();
    window_b.bind(&app);

    window_a.send(window::SetSize(PhysicalSize::new(500, 500)));
}
```

## Getting started

Instatllation is simple with:

```sh
cargo add viewbuilder --features full
```
