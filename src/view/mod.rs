use crate::Context;
use std::{marker::PhantomData, rc::Rc};

mod adapt;
pub use self::adapt::Adapt;

pub trait View<M, A> {
    type Element;

    fn build(&mut self, cx: &mut Context<M, A>) -> Self::Element;

    fn rebuild(&mut self, cx: &mut Context<M, A>, element: &mut Self::Element);

    fn adapt<M2, F, A2>(self, f: F) -> Adapt<F, Self, M, A>
    where
        Self: Sized,
        F: Fn(&mut M2, &dyn Fn(&mut M) -> Option<A>) -> Option<A2> + 'static,
        M: 'static,
    {
        Adapt {
            f: Rc::new(f),
            child: self,
            _marker: PhantomData,
        }
    }
}
