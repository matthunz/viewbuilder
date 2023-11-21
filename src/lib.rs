use slotmap::{new_key_type, DefaultKey, SlotMap, SparseSecondaryMap};
use std::{any::Any, borrow::Cow, marker::PhantomData};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

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
    pub fn get(self, tree: &LocalTree) -> &E
    where
        E: 'static,
    {
        tree.elements[self.key].as_any().downcast_ref().unwrap()
    }

    pub fn get_mut(self, tree: &mut LocalTree) -> &mut E
    where
        E: 'static,
    {
        tree.elements[self.key].as_any_mut().downcast_mut().unwrap()
    }

    pub fn send(self, tree: &mut LocalTree, msg: E::Message)
    where
        E: Element + 'static,
    {
        tree.elements[self.key].handle_any(Box::new(msg));
        tree.tx.send(tree.key).unwrap();
    }

    pub fn push_child(self, tree: &mut LocalTree, key: DefaultKey) {
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

pub struct LocalTree {
    key: TreeKey,
    tx: UnboundedSender<TreeKey>,
    elements: SlotMap<DefaultKey, Box<dyn AnyElement>>,
    children: SparseSecondaryMap<DefaultKey, Vec<DefaultKey>>,
    parents: SparseSecondaryMap<DefaultKey, DefaultKey>,
}

impl LocalTree {
    pub fn builder() -> LocalTreeBuilder {
        LocalTreeBuilder {}
    }

    pub fn insert<E: Element + 'static>(&mut self, element: E) -> ElementRef<E> {
        let key = self.elements.insert(Box::new(element));

        ElementRef {
            key,
            _marker: PhantomData,
        }
    }
}

pub enum TreeMessage {
    Handle { key: DefaultKey, msg: Box<dyn Any> },
}

impl Element for LocalTree {
    type Message = TreeMessage;

    fn handle(&mut self, msg: Self::Message) {
        match msg {
            TreeMessage::Handle { key, msg } => self.elements[key].handle_any(msg),
        }
    }
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

    fn handle(&mut self, msg: Self::Message) {}
}

pub trait TreeBuilder {
    type Tree: 'static;

    fn insert_with_key(self, key: TreeKey, tx: UnboundedSender<TreeKey>) -> Self::Tree;
}

pub struct LocalTreeBuilder {}

impl TreeBuilder for LocalTreeBuilder {
    type Tree = LocalTree;

    fn insert_with_key(self, key: TreeKey, tx: UnboundedSender<TreeKey>) -> Self::Tree {
        LocalTree {
            key,
            tx,
            elements: Default::default(),
            children: Default::default(),
            parents: Default::default(),
        }
    }
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

pub struct UserInterface {
    trees: SlotMap<TreeKey, Box<dyn Any>>,
    tx: UnboundedSender<TreeKey>,
    rx: UnboundedReceiver<TreeKey>,
}

impl UserInterface {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        Self {
            trees: SlotMap::default(),
            tx,
            rx,
        }
    }

    pub fn insert<T: 'static>(&mut self, tree_builder: impl TreeBuilder<Tree = T>) -> TreeRef<T> {
        let key = self
            .trees
            .insert_with_key(|key| Box::new(tree_builder.insert_with_key(key, self.tx.clone())));
        TreeRef {
            key,
            _marker: PhantomData,
        }
    }
}
