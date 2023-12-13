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
use concoct::{Context, Handle, Object, Slot};
use viewbuilder::{
    view::{LinearLayout, Text},
    window, UserInterface, Window,
};
use winit::dpi::PhysicalSize;

struct App {
    width_text: Handle<Text>,
    height_text: Handle<Text>,
    size: PhysicalSize<u32>,
}

impl Object for App {}

impl Slot<window::Resized> for App {
    fn handle(&mut self, _cx: Context<Self>, msg: window::Resized) {
        if msg.width != self.size.width {
            self.width_text.send(format!("Width: {}", msg.width).into());
            self.size.width = msg.width
        }

        if msg.height != self.size.height {
            self.height_text
                .send(format!("Height: {}", msg.height).into());
            self.size.height = msg.height
        }
    }
}

#[viewbuilder::main]
fn main() {
    let width_text = Text::default().spawn();
    let height_text = Text::default().spawn();

    let app = App {
        width_text: width_text.clone(),
        height_text: height_text.clone(),
        size: PhysicalSize::default(),
    }
    .spawn();

    let window = Window::new(LinearLayout::new((width_text, height_text))).spawn();
    window.bind(&app);
}
```

## Getting started

Instatllation is simple with:

```sh
cargo add viewbuilder --features full
```
