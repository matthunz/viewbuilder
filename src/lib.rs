use kurbo::Size;
use slotmap::{DefaultKey, SlotMap};
use std::{any::Any, cell::RefCell, collections::HashSet, marker::PhantomData, mem, rc::Rc};

pub trait Element {
    type Message;

    fn update(&mut self, cx: Handle<Self>, msg: Self::Message);

    fn layout(&mut self, min_size: Option<Size>, max_size: Option<Size>) -> Size;
}

pub trait AnyElement {
    fn update_any(&mut self, key: DefaultKey, ui: UserInterface, msg: Box<dyn Any>);

    fn layout_any(&mut self, min_size: Option<Size>, max_size: Option<Size>) -> Size;
}

impl<E: Element> AnyElement for E
where
    E::Message: 'static,
{
    fn update_any(&mut self, key: DefaultKey, ui: UserInterface, msg: Box<dyn Any>) {
        let cx = Handle {
            key,
            ui,
            _marker: PhantomData,
        };
        self.update(cx, *msg.downcast().unwrap())
    }

    fn layout_any(&mut self, min_size: Option<Size>, max_size: Option<Size>) -> Size {
        self.layout(min_size, max_size)
    }
}

pub struct Node {
    element: Rc<RefCell<dyn AnyElement>>,
}

pub struct Handle<E: ?Sized> {
    key: DefaultKey,
    ui: UserInterface,
    _marker: PhantomData<E>,
}

impl<E> Handle<E> {
    pub fn send(&self, msg: E::Message)
    where
        E: Element,
        E::Message: 'static,
    {
        self.ui
            .inner
            .borrow_mut()
            .queue
            .push((self.key, Box::new(msg)));
    }

    pub fn layout(&self) {
        self.ui.inner.borrow_mut().pending_layouts.insert(self.key);
    }
}

#[derive(Default)]
struct Inner {
    queue: Vec<(DefaultKey, Box<dyn Any>)>,
    nodes: SlotMap<DefaultKey, Node>,
    pending_layouts: HashSet<DefaultKey>,
}

#[derive(Clone, Default)]
pub struct UserInterface {
    inner: Rc<RefCell<Inner>>,
}

impl UserInterface {
    pub fn insert<E: Element + 'static>(&self, element: E) -> Handle<E> {
        let node = Node {
            element: Rc::new(RefCell::new(element)),
        };
        let key = self.inner.borrow_mut().nodes.insert(node);
        Handle {
            key,
            ui: self.clone(),
            _marker: PhantomData,
        }
    }

    pub fn run(&self) {
        let mut queue = mem::take(&mut self.inner.borrow_mut().queue);
        while let Some((key, msg)) = queue.pop() {
            let element = self.inner.borrow().nodes[key].element.clone();
            element.borrow_mut().update_any(key, self.clone(), msg);
        }

        let pending_layouts = mem::take(&mut self.inner.borrow_mut().pending_layouts);
        for key in pending_layouts {
            let element = self.inner.borrow().nodes[key].element.clone();
            element.borrow_mut().layout_any(None, None);
        }
    }
}

pub enum TextMessage {
    Set,
}

pub struct Text {}

impl Element for Text {
    type Message = TextMessage;

    fn update(&mut self, cx: Handle<Self>, msg: Self::Message) {
        cx.layout();
    }

    fn layout(&mut self, min_size: Option<Size>, max_size: Option<Size>) -> Size {
        dbg!("layout");
        Size::default()
    }
}
