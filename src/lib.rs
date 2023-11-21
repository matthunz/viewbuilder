use slotmap::{DefaultKey, SlotMap, SparseSecondaryMap};
use std::{any::Any, borrow::Cow, marker::PhantomData};

pub trait Element {
    type Message;

    fn handle(&mut self, msg: Self::Message);
}

pub trait AnyElement {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn handle_any(&mut self, msg: Box<dyn Any>);
}

impl<E> AnyElement for E
where
    E: Element + 'static,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn handle_any(&mut self, msg: Box<dyn Any>) {
        self.handle(*msg.downcast().unwrap())
    }
}

pub struct ElementRef<E> {
    key: DefaultKey,
    _marker: PhantomData<E>,
}

impl<E> Clone for ElementRef<E> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<E> Copy for ElementRef<E> {}

impl<E> ElementRef<E> {
    pub fn get(self, tree: &Tree) -> &E
    where
        E: 'static,
    {
        tree.elements[self.key].as_any().downcast_ref().unwrap()
    }

    pub fn get_mut(self, tree: &mut Tree) -> &mut E
    where
        E: 'static,
    {
        tree.elements[self.key].as_any_mut().downcast_mut().unwrap()
    }

    pub fn send(self, tree: &mut Tree, msg: E::Message)
    where
        E: Element + 'static,
    {
        tree.elements[self.key].handle_any(Box::new(msg))
    }

    pub fn push_child(self, tree: &mut Tree, key: DefaultKey) {
        if let Some(children) = tree.children.get_mut(self.key) {
            children.push(key);
        } else {
            tree.children.insert(self.key, vec![key]);
        }

        if let Some(parent) = tree.parents.get_mut(key) {
            // Remove this key's previous parent (if it exists).
            if let Some(children) = tree.children.get_mut(*parent) {
                if let Some(idx) = children.iter().position(|child_key| key == *child_key) {
                    children.remove(idx);
                }
            }

            *parent = self.key;
        } else {
            tree.parents.insert(key, self.key);
        }
    }
}

#[derive(Default)]
pub struct Tree {
    elements: SlotMap<DefaultKey, Box<dyn AnyElement>>,
    children: SparseSecondaryMap<DefaultKey, Vec<DefaultKey>>,
    parents: SparseSecondaryMap<DefaultKey, DefaultKey>,
}

impl Tree {
    pub fn insert<E: Element + 'static>(&mut self, element: E) -> ElementRef<E> {
        let key = self.elements.insert(Box::new(element));

        ElementRef {
            key,
            _marker: PhantomData,
        }
    }
}

impl Element for Tree {
    type Message = ();

    fn handle(&mut self, msg: Self::Message) {}
}

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
        tree: &mut Tree,
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
