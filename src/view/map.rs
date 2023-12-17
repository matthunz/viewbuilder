use crate::{ViewBuilder, Context};
use std::{marker::PhantomData, sync::Arc};

/// View for the `View::map` method.
pub struct Map<V, F, M> {
    pub(super) view: V,
    pub(super) f: Arc<F>,
    pub(super) _marker: PhantomData<M>,
}

impl<V, F, M1, M2> ViewBuilder<M1> for Map<V, F, M2>
where
    V: ViewBuilder<M2>,
    F: Fn(M2) -> M1 + Send + Sync + 'static,
    M1: Send + 'static,
{
    type View = V::View;

    fn build(&mut self, cx: &mut Context<M1>) -> Self::View {
        let f = self.f.clone();
        let send = cx.send.clone();
        let mut cx = Context::new(Arc::new(move |msg| send(f(msg))));

        self.view.build(&mut cx)
    }

    fn rebuild(&mut self, cx: &mut Context<M1>, state: &mut Self::View) {
        let f = self.f.clone();
        let send = cx.send.clone();
        let mut cx = Context::new(Arc::new(move |msg| send(f(msg))));

        self.view.rebuild(&mut cx, state)
    }
}
