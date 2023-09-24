# Viewbuilder

Cross-platform user interface framework for Rust.

This crate provides an HTML-like render API for the backend of a UI.
You can either use [concoct](https://github.com/concoct-rs/concoct), bring your own state management tools, or build your own framework using this as a backend.

## Features
 - Cross-platform with desktop and mobile support
 - HTML-like API
 - CSS flexbox and grid layout with [taffy](https://github.com/DioxusLabs/taffy/)
 - Accessibility with [accesskit](https://github.com/AccessKit/accesskit)
 - High performance rendering with [rust-skia](https://github.com/rust-skia/rust-skia)

## Example
```rust
let mut tree = Tree::default();

let inc_count = Rc::new(AtomicI64::new(0));
let dec_count = inc_count.clone();

let text = tree.insert("0");
let root = Element::builder()
    .flex_direction(FlexDirection::Column)
    .child(Element::builder().child(text).build(&mut tree))
    .child(
        Element::builder()
            .flex_direction(FlexDirection::Row)
            .child(
                Element::builder()
                    .on_click(Box::new(move |tree, _event| {
                        inc_count.fetch_add(1, Ordering::SeqCst);
                        tree.set_text(text, inc_count.load(Ordering::SeqCst).to_string())
                    }))
                    .background_color(Color4f::new(1., 1., 0., 1.))
                    .child(tree.insert("More!"))
                    .build(&mut tree),
            )
            .child(
                Element::builder()
                    .on_click(Box::new(move |tree, _event| {
                        dec_count.fetch_sub(1, Ordering::SeqCst);
                        tree.set_text(text, dec_count.load(Ordering::SeqCst).to_string())
                    }))
                    .background_color(Color4f::new(1., 1., 0., 1.))
                    .child(tree.insert("Less!"))
                    .build(&mut tree),
            )
            .build(&mut tree),
    )
    .build(&mut tree);

viewbuilder::run(tree, root)
```
