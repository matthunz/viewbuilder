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
use viewbuilder::{
    view,
    web::{self, html, Web},
    ControlFlow, Model, View,
};

enum Message {
    Increment,
    Decrement,
}

#[derive(Default)]
struct App {
    count: i32,
}

impl Model<Message> for App {
    fn handle(&mut self, msg: Message) -> ControlFlow {
        match msg {
            Message::Decrement => self.count -= 1,
            Message::Increment => self.count += 1,
        }
        ControlFlow::Rebuild
    }
}

fn view(model: &App) -> impl View<Web, Message> {
    (
        format!("High five count: {}", model.count),
        view::once(html::button(
            html::on_click(|| Message::Increment),
            "Up high!",
        )),
        view::once(html::button(
            html::on_click(|| Message::Decrement),
            "Down low!",
        )),
    )
}

fn main() {
    web::run(App::default(), view)
}
```

## Getting started

Instatllation is simple with:

```sh
cargo add viewbuilder --features full
```
