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
let ui = UserInterface::new();
let tree = ui.insert(LocalTree::builder(Window::default()));

let text = tree.insert(Text::new("Hello World!"));
tree.root().push_child(text.key);

ui.run();
```

## Features
- Cross-platform with desktop and mobile support
- Event handling with an HTML-like API
- State management with [dioxus](https://github.com/DioxusLabs/dioxus/) (optional)
- CSS flexbox and grid layout with [taffy](https://github.com/DioxusLabs/taffy/)
- Accessibility with [accesskit](https://github.com/AccessKit/accesskit)
- High performance rendering with [rust-skia](https://github.com/rust-skia/rust-skia)

## Getting started
Instatllation is simple with:
```sh
cargo add viewbuilder --features full
```
If you encounter errors, please check the instructions for building [rust-skia](https://github.com/rust-skia/rust-skia).

