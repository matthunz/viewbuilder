use crate::{Context, View};
use std::{marker::PhantomData, sync::Arc};

/// View for the `View::map` method.
pub struct Map<V, F, M> {
    pub(super) view: V,
    pub(super) f: Arc<F>,
    pub(super) _marker: PhantomData<M>,
}

impl<V, F, T, M1, M2> View<T, M1> for Map<V, F, M2>
where
    V: View<T, M2>,
    F: Fn(M2) -> M1 + Send + Sync + 'static,
    M1: Send + 'static,
{
    type Element = V::Element;

    fn build(&mut self, cx: &mut Context<M1>, tree: &mut T) -> Self::Element {
        let f = self.f.clone();
        let send = cx.send.clone();
        let mut cx = Context::new(Arc::new(move |msg| send(f(msg))));

        self.view.build(&mut cx, tree)
    }

    fn rebuild(&mut self, cx: &mut Context<M1>, tree: &mut T, element: &mut Self::Element) {
        let f = self.f.clone();
        let send = cx.send.clone();
        let mut cx = Context::new(Arc::new(move |msg| send(f(msg))));

        self.view.rebuild(&mut cx, tree, element)
    }

    fn remove(&mut self, cx: &mut Context<M1>, state: &mut T, element: Self::Element) {
        let f = self.f.clone();
        let send = cx.send.clone();
        let mut cx = Context::new(Arc::new(move |msg| send(f(msg))));

        self.view.remove(&mut cx, state, element)
    }
}
