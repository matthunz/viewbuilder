use std::{mem, sync::Arc};
#[cfg(feature = "tokio")]
use tokio::sync::mpsc;
use web_sys::{
    wasm_bindgen::{closure::Closure, JsCast},
    Document, Element, Text,
};

pub mod view;
pub use self::view::View;

pub mod web;

pub struct Context<M> {
    send: Arc<dyn Fn(M)>,
}

impl<M> Context<M> {
    pub fn new(send: Arc<dyn Fn(M)>) -> Self {
        Self { send }
    }

    pub fn send(&self, msg: M) {
        (self.send)(msg)
    }
}

impl<M> Clone for Context<M> {
    fn clone(&self) -> Self {
        Self {
            send: self.send.clone(),
        }
    }
}

pub enum ControlFlow {
    Pending,
    Rebuild,
}

pub trait Model<M> {
    fn handle(&mut self, msg: M) -> ControlFlow;
}

pub struct Application<T, F, S, M, Tree> {
    model: T,
    composable: F,
    state: Option<S>,
    cx: Context<M>,
    tree: Tree,
}

impl<T, F, S, M, Tree> Application<T, F, S, M, Tree> {
    pub fn new(send: Arc<dyn Fn(M)>, model: T, composable: F, tree: Tree) -> Self
    where
        M: Send + 'static,
    {
        let cx = Context::new(send);

        Self {
            model,
            composable,
            state: None,
            cx,

            tree,
        }
    }

    pub fn build<C>(&mut self)
    where
        T: Model<M>,
        F: FnMut(&T) -> C,
        C: View<Tree, M, Element = S>,
    {
        let state = (self.composable)(&self.model).build(&mut self.cx, &mut self.tree);
        self.state = Some(state);
    }

    pub fn rebuild<C>(&mut self)
    where
        T: Model<M>,
        F: FnMut(&T) -> C,
        C: View<Tree, M, Element = S>,
    {
        let state = self.state.as_mut().unwrap();
        (self.composable)(&self.model).rebuild(&mut self.cx, &mut self.tree, state);
    }

    pub fn handle(&mut self, msg: M) -> ControlFlow
    where
        T: Model<M>,
        M: 'static,
    {
        self.model.handle(msg)
    }
}
#[cfg(feature = "tokio")]
pub struct App<T, F, S, M, Tree> {
    model: T,
    composable: F,
    state: Option<S>,
    cx: Context<M>,
    tree: Tree,
    rx: mpsc::UnboundedReceiver<M>,
}

#[cfg(feature = "tokio")]
impl<T, F, S, M, Tree> App<T, F, S, M, Tree> {
    pub fn new(model: T, composable: F, tree: Tree) -> Self
    where
        M: Send + 'static,
    {
        let (tx, rx) = mpsc::unbounded_channel();
        let cx = Context::new(Arc::new(move |msg| {
            tx.send(msg).unwrap();
        }));
        Self {
            model,
            composable,
            state: None,
            cx,
            rx,
            tree,
        }
    }

    pub fn build<C>(&mut self)
    where
        T: Model<M>,
        F: FnMut(&T) -> C,
        C: View<Tree, M, Element = S>,
    {
        let state = (self.composable)(&self.model).build(&mut self.cx, &mut self.tree);
        self.state = Some(state);
    }

    pub fn rebuild<C>(&mut self)
    where
        T: Model<M>,
        F: FnMut(&T) -> C,
        C: View<Tree, M, Element = S>,
    {
        let state = self.state.as_mut().unwrap();
        (self.composable)(&self.model).rebuild(&mut self.cx, &mut self.tree, state);
    }

    pub async fn handle(&mut self) -> ControlFlow
    where
        T: Model<M>,
        M: 'static,
    {
        let msg = self.rx.recv().await.unwrap();
        self.model.handle(msg)
    }

    pub fn try_handle(&mut self) -> Option<ControlFlow>
    where
        T: Model<M>,
        M: 'static,
    {
        if let Ok(msg) = self.rx.try_recv() {
            Some(self.model.handle(msg))
        } else {
            None
        }
    }

    pub fn try_run<C>(&mut self)
    where
        T: Model<M>,
        F: FnMut(&T) -> C,
        C: View<Tree, M, Element = S>,
        M: 'static,
    {
        match self.try_handle() {
            Some(ControlFlow::Rebuild) => self.rebuild(),
            Some(ControlFlow::Pending) | None => {}
        }
    }

    pub async fn run<C>(&mut self)
    where
        T: Model<M>,
        F: FnMut(&T) -> C,
        C: View<Tree, M, Element = S>,
        M: 'static,
    {
        match self.handle().await {
            ControlFlow::Rebuild => self.rebuild(),
            ControlFlow::Pending => {}
        }
    }
}

pub struct Web {
    document: Document,
    parent: Element,
}

impl Default for Web {
    fn default() -> Self {
        let document = web_sys::window().unwrap().document().unwrap();
        Self {
            parent: document.body().unwrap().unchecked_into(),
            document,
        }
    }
}

pub struct HtmlAttributes {
    element: Element,
}

impl<M> View<Web, M> for &'static str {
    type Element = (Self, Text);

    fn build(&mut self, _cx: &mut Context<M>, tree: &mut Web) -> Self::Element {
        let text = tree.document.create_text_node(self);
        tree.parent.append_child(&text).unwrap();
        (self, text)
    }

    fn rebuild(&mut self, _cx: &mut Context<M>, _tree: &mut Web, element: &mut Self::Element) {
        if *self != element.0 {
            element.0 = self;
            element.1.set_text_content(Some(self));
        }
    }
}

impl<M> View<Web, M> for String {
    type Element = (Self, Text);

    fn build(&mut self, _cx: &mut Context<M>, tree: &mut Web) -> Self::Element {
        let text = tree.document.create_text_node(self);
        tree.parent.append_child(&text).unwrap();
        (self.clone(), text)
    }

    fn rebuild(&mut self, _cx: &mut Context<M>, _tree: &mut Web, element: &mut Self::Element) {
        if *self != element.0 {
            element.0 = self.clone();
            element.1.set_text_content(Some(self));
        }
    }
}
