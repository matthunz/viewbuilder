use slotmap::new_key_type;
use std::borrow::Cow;

mod any_element;
pub use any_element::AnyElement;

pub mod element;
pub use element::Element;

mod element_ref;
pub use element_ref::LocalElementRef;

pub mod tree;
pub use tree::LocalTree;

mod ui;
pub use ui::UserInterface;

pub enum TextMessage {
    SetContent(Cow<'static, str>),
}

pub struct Text {
    content: Cow<'static, str>,
}

impl Text {
    pub fn new(content: impl Into<Cow<'static, str>>) -> Self {
        Self {
            content: content.into(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn set_content<R>(text: LocalElementRef<R, Self>, content: impl Into<Cow<'static, str>>) {
        text.send(TextMessage::SetContent(content.into()))
    }
}

impl Element for Text {
    type Message = TextMessage;

    fn handle(&mut self, msg: Self::Message) {
        match msg {
            TextMessage::SetContent(content) => {
                self.content = content;
            }
        }
    }

    fn render(&mut self, _scene: vello::SceneBuilder) {
        dbg!(&self.content);
    }
}

new_key_type! {
    pub struct TreeKey;
}
