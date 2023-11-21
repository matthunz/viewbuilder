use slotmap::new_key_type;
use std::{borrow::Cow, marker::PhantomData};

mod any_element;
pub use any_element::AnyElement;

mod element;
pub use element::Element;

mod element_ref;
pub use element_ref::ElementRef;

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

    pub fn set_content(
        text: ElementRef<Self>,
        tree: &mut LocalTree,
        content: impl Into<Cow<'static, str>>,
    ) {
        text.send(tree, TextMessage::SetContent(content.into()))
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
}

new_key_type! {
    pub struct TreeKey;
}

impl<T> Element for TreeRef<T> {
    type Message = ();

    fn handle(&mut self, _msg: Self::Message) {}
}

pub struct TreeRef<T> {
    key: TreeKey,
    _marker: PhantomData<T>,
}

impl<T> Clone for TreeRef<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for TreeRef<T> {}

impl<T: 'static> TreeRef<T> {
    pub fn get_mut(self, ui: &mut UserInterface) -> &mut T {
        ui.trees[self.key].downcast_mut().unwrap()
    }
}
