# Viewbuilder

[Examples](https://github.com/matthunz/viewbuilder/tree/main/examples)




Cross-platform user interface framework for Rust.

This crate provides an HTML-like render API for the backend of a UI.
It's built for use as a backend for [concoct](https://github.com/concoct-rs/concoct),
but you can bring your own state management tools or build your own framework using this as a backend.

## Features
 - Cross-platform with desktop and mobile support
 - HTML-like API
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
