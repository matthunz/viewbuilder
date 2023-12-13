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

## Web
```rust
use concoct::{Handle, Object, Signal, Slot};
use viewbuilder::web::{Element, Text};

#[derive(Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

#[derive(Default)]
struct Counter {
    value: i32,
}

impl Object for Counter {}

impl Signal<i32> for Counter {}

impl Slot<Message> for Counter {
    fn handle(&mut self, cx: Handle<Self>, msg: Message) {
        match msg {
            Message::Increment => self.value += 1,
            Message::Decrement => self.value -= 1,
        };
        cx.emit(self.value);
    }
}

fn counter_button(counter: &Handle<Counter>, label: &str, msg: Message) -> Handle<Element> {
    let button = Element::builder().child(Text::new(label)).build().start();
    button.map(&counter, move |_| msg);
    button
}

#[viewbuilder::main]
fn main() {
    let text = Text::new("0").start();

    let counter = Counter::default().start();
    counter.map(&text, |value| value.to_string());

    Element::builder()
        .child((
            text,
            counter_button(&counter, "Up high!", Message::Increment),
            counter_button(&counter, "Down low!", Message::Decrement),
        ))
        .build()
        .start();
}
```

## Getting started

Instatllation is simple with:

```sh
cargo add viewbuilder --features full
```
