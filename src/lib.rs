use slotmap::{DefaultKey, SlotMap};
use std::{
    cell::{Ref, RefCell, RefMut},
    marker::PhantomData,
    rc::Rc,
};

pub mod element;
pub use self::element::{AnyElement, Element};

mod view;
pub use self::view::View;

mod window;
pub use self::window::Window;

pub struct Node {
    element: Rc<RefCell<dyn AnyElement>>,
}

pub struct Entry<E> {
    element: Rc<RefCell<dyn AnyElement>>,
    _marker: PhantomData<E>,
}

impl<E: 'static> Entry<E> {
    pub fn borrow(&self) -> Ref<E> {
        Ref::map(self.element.borrow(), |element| {
            element.as_any().downcast_ref().unwrap()
        })
    }

    pub fn borrow_mut(&self) -> RefMut<E> {
        RefMut::map(self.element.borrow_mut(), |element| {
            element.as_any_mut().downcast_mut().unwrap()
        })
    }
}

pub struct ElementRef<E> {
    pub key: DefaultKey,
    _marker: PhantomData<E>,
}

impl<E> ElementRef<E> {
    pub fn get(self) -> Entry<E> {
        let element = UserInterface::current().inner.borrow_mut().nodes[self.key]
            .element
            .clone();
        Entry {
            element,
            _marker: PhantomData,
        }
    }
}

#[derive(Default)]
struct Inner {
    nodes: SlotMap<DefaultKey, Node>,
}

#[derive(Clone, Default)]
pub struct UserInterface {
    inner: Rc<RefCell<Inner>>,
}

impl UserInterface {
    pub fn current() -> Self {
        thread_local! {
            static CURRENT: UserInterface = UserInterface::default()
        }
        CURRENT.try_with(|ui| ui.clone()).unwrap()
    }

    pub fn view<E: Element + 'static>(&self, element: E) -> ElementRef<E> {
        let node = Node {
            element: Rc::new(RefCell::new(element)),
        };
        let key = self.inner.borrow_mut().nodes.insert(node);
        ElementRef {
            key,
            _marker: PhantomData,
        }
    }
}

pub fn view<E: Element + 'static>(element: E) -> ElementRef<E> {
    UserInterface::current().view(element)
}
