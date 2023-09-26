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

This crate provides an HTML-like render API for the backend of a UI.
It's built for use as a backend for [concoct](https://github.com/concoct-rs/concoct),
but you can bring your own state management tools or build your own framework using this as a backend.

## Features

- Cross-platform with desktop and mobile support
- Event handling with an HTML-like API
- CSS flexbox and grid layout with [taffy](https://github.com/DioxusLabs/taffy/)
- Accessibility with [accesskit](https://github.com/AccessKit/accesskit)
- High performance rendering with [rust-skia](https://github.com/rust-skia/rust-skia)

## Examples

### Hello World
```rust
fn app(cx: &mut Context) -> NodeKey {
    Element::new()
        .align_items(AlignItems::Center)
        .justify_content(JustifyContent::Center)
        .child(cx.insert("Hello World!"))
        .build(cx)
}

fn main() {
    viewbuilder::run(app)
}
```

### Scroll
```rust
fn app(cx: &mut Context) -> NodeKey {
    let mut elem = Element::new();
    elem.overflow_y(Overflow::Scroll)
        .flex_direction(FlexDirection::Column)
        .extend((0..100).map(|count| cx.insert(count.to_string())));
    elem.build(cx)
}
```

### Button Component
```rust
fn button(
    cx: &mut Context,
    label: &'static str,
    mut handler: impl FnMut(&mut Context) + 'static,
) -> NodeKey {
    Element::new()
        .on_click(Box::new(move |cx, _event| handler(cx)))
        .background_color(Color4f::new(1., 1., 0., 1.))
        .child(cx.insert(label))
        .build(cx)
}
```
