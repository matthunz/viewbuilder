use std::{cell::RefCell, collections::VecDeque, mem, rc::Rc, sync::Arc};
#[cfg(feature = "tokio")]
use tokio::sync::mpsc;
use web_sys::{wasm_bindgen::JsCast, Document, Element, HtmlElement};

pub mod view;
pub use self::view::View;

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
    queue: Rc<RefCell<VecDeque<M>>>,
}

impl<T, F, S, M, Tree> Application<T, F, S, M, Tree> {
    pub fn new(model: T, composable: F, tree: Tree) -> Self
    where
        M: Send + 'static,
    {
        let queue = Rc::new(RefCell::new(VecDeque::new()));
        let queue_tx = queue.clone();

        let cx = Context::new(Arc::new(move |msg| {
            queue_tx.borrow_mut().push_front(msg);
        }));

        Self {
            model,
            composable,
            state: None,
            cx,
            queue,
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

    pub fn try_handle(&mut self) -> Option<ControlFlow>
    where
        T: Model<M>,
        M: 'static,
    {
        if let Some(msg) = self.queue.borrow_mut().pop_back() {
            Some(self.model.handle(msg))
        } else {
            None
        }
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

pub fn div<A, C, M>(attrs: A, content: C) -> Div<A, C>
where
    A: View<HtmlAttributes, M>,
    C: View<Web, M>,
{
    Div { attrs, content }
}

pub struct Div<A, C> {
    attrs: A,
    content: C,
}

impl<M, A, C> View<Web, M> for Div<A, C>
where
    A: View<HtmlAttributes, M>,
    C: View<Web, M>,
{
    type Element = (HtmlAttributes, A::Element);

    fn build(&mut self, cx: &mut Context<M>, tree: &mut Web) -> Self::Element {
        let element = tree.document.create_element("div").unwrap();
        tree.parent.append_child(&element).unwrap();

        let parent = mem::replace(&mut tree.parent, element);
        self.content.build(cx, tree);
        let element = mem::replace(&mut tree.parent, parent);

        let mut element_attrs = HtmlAttributes { element };
        let attrs = self.attrs.build(cx, &mut element_attrs);
        (element_attrs, attrs)
    }

    fn rebuild(&mut self, cx: &mut Context<M>, tree: &mut Web, element: &mut Self::Element) {
        self.attrs.rebuild(cx, &mut element.0, &mut element.1)
    }
}

pub fn class<M>(name: impl AsRef<str> + 'static) -> impl View<HtmlAttributes, M> {
    view::from_fn(move |_cx, tree: &mut HtmlAttributes| {
        tree.element.set_class_name(name.as_ref());
    })
}
