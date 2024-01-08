use crate::{Context, View};

pub fn from_fn<F, T, M>(f: F) -> FromFn<F>
where
    F: FnMut(&mut Context<M>, &mut T),
{
    FromFn { f }
}

pub struct FromFn<F> {
    f: F,
}

impl<T, M, F> View<T, M> for FromFn<F>
where
    F: FnMut(&mut Context<M>, &mut T),
{
    type Element = ();

    fn build(&mut self, cx: &mut Context<M>, tree: &mut T) -> Self::Element {
        (self.f)(cx, tree)
    }

    fn rebuild(&mut self, cx: &mut Context<M>, tree: &mut T, _state: &mut Self::Element) {
        (self.f)(cx, tree)
    }
}
