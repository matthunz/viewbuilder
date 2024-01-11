use std::{marker::PhantomData, rc::Rc};

mod view;
pub use self::view::View;

mod element;
pub use self::element::Element;

pub struct Waker<M, A> {
    updater: Rc<dyn Fn(&dyn Fn(&mut M) -> Option<A>)>,
}

impl<M, A> Waker<M, A> {
    pub fn new(updater: Rc<dyn Fn(&dyn Fn(&mut M) -> Option<A>)>) -> Self {
        Self { updater }
    }

    pub fn wake(self, update: &dyn Fn(&mut M) -> Option<A>) {
        (self.updater)(update)
    }
}

impl<M, A> Clone for Waker<M, A> {
    fn clone(&self) -> Self {
        Self {
            updater: self.updater.clone(),
        }
    }
}

pub struct Context<'a, M, A> {
    pub waker: &'a Waker<M, A>,
}
