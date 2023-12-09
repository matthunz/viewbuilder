use crate::{Element, LocalElementRef};
use std::borrow::Cow;

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

    pub fn set_content(text: LocalElementRef<Self>, content: impl Into<Cow<'static, str>>) {
        text.send(TextMessage::SetContent(content.into()))
    }
}

impl Element for Text {
    type Message = TextMessage;

    fn lifecycle(&mut self, _cx: super::LifecycleContext, _lifecycle: super::Lifecycle) {}

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
