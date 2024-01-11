use std::{marker::PhantomData, rc::Rc};

mod view;
pub use self::view::View;

pub struct Waker<M, A> {
    updater: Rc<dyn Fn(&dyn Fn(&mut M) -> Option<A>)>,
}

impl<M, A> Waker<M, A> {
    pub fn new(updater: Rc<dyn Fn(&dyn Fn(&mut M) -> Option<A>)>,) -> Self {
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

impl<M, A> View<M, A> for () {
    type Element = ();

    fn build(&mut self, cx: &mut Context<M, A>) -> Self::Element {}

    fn rebuild(&mut self, cx: &mut Context<M, A>, element: &mut Self::Element) {}
}
