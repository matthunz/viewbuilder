<div align="center">
<h1>Viewbuilder</h1>
 <a href="https://crates.io/crates/viewbuilder">
    <img src="https://img.shields.io/crates/v/viewbuilder?style=flat-square"
    alt="Crates.io version" />
  </a>
  <a href="https://docs.rs/viewbuilder">
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

This crate provides an HTML-like render API for the backend of a UI.
It's built for use as a backend for [concoct](https://github.com/concoct-rs/concoct),
but you can bring your own state management tools or build your own framework using this as a backend.

## Features

- Cross-platform with desktop and mobile support
- Event handling with an HTML-like API
- CSS flexbox and grid layout with [taffy](https://github.com/DioxusLabs/taffy/)
- Accessibility with [accesskit](https://github.com/AccessKit/accesskit)
- High performance rendering with [rust-skia](https://github.com/rust-skia/rust-skia)

```rust
let mut tree = Tree::default();
let root = Element::builder()
    .align_items(AlignItems::Center)
    .justify_content(JustifyContent::Center)
    .child(tree.insert("Hello World!"))
    .build(&mut tree);

viewbuilder::run(tree, root)
```

```rust
fn button(tree: &mut Tree, mut f: impl FnMut(&mut Tree) + 'static) -> DefaultKey {
    Element::builder()
        .on_click(Box::new(move |tree, _event| f(tree)))
        .background_color(Color4f::new(1., 1., 0., 1.))
        .child(tree.insert("More!"))
        .build(tree)
}
```