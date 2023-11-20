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
enum Message {
    Increment,
    Decrement,
}

#[derive(Default)]
struct Counter {
    count: i32,
}

impl Component for Counter {
    type Message = Message;

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Message::Increment => self.count += 1,
            Message::Decrement => self.count -= 1,
        }
    }

    fn view<'a>(&mut self, bump: &'a Bump) -> impl View<'a, Self::Message> {
        LinearLayout::new((
            format_in!(bump, "High five count: {}", self.count),
            LinearLayout::new((
                Text::new("Up high!").on_click(Message::Increment),
                Text::new("Down low!").on_click(Message::Decrement),
            )),
        ))
    }
}
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

