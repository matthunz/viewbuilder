use crate::{Context, View, Waker};
use std::{marker::PhantomData, rc::Rc};

pub struct Adapt<F, V, M, A> {
    pub(crate) f: Rc<F>,
    pub(crate) child: V,
    pub(crate) _marker: PhantomData<(M, A)>,
}

impl<F, V, M2, A2> Adapt<F, V, M2, A2> {
    fn with_cx<M1, A1, R>(
        &mut self,
        cx: &mut Context<M1, A1>,
        with: impl FnOnce(&mut Self, &mut Context<M2, A2>) -> R,
    ) -> R
    where
        F: Fn(&mut M1, &dyn Fn(&mut M2) -> Option<A2>) -> Option<A1> + 'static,
        V: View<M2, A2>,
        M1: 'static,
        A1: 'static,
    {
        let waker = cx.waker.clone();
        let f = self.f.clone();

        let child_waker = Waker::new(Rc::new(move |g| {
            let waker = waker.clone();
            let f = f.clone();
            waker.wake(&|model| f(model, g));
        }));
        
        let mut cx = Context {
            waker: &child_waker,
        };
        with(self, &mut cx)
    }
}

impl<M1, M2, A1, A2, F, V> View<M1, A1> for Adapt<F, V, M2, A2>
where
    F: Fn(&mut M1, &dyn Fn(&mut M2) -> Option<A2>) -> Option<A1> + 'static,
    V: View<M2, A2>,
    M1: 'static,
    A1: 'static,
{
    type Element = V::Element;

    fn build(&mut self, cx: &mut Context<M1, A1>) -> Self::Element {
        self.with_cx(cx, |me, cx| me.child.build(cx))
    }

    fn rebuild(&mut self, cx: &mut Context<M1, A1>, element: &mut Self::Element) {
        self.with_cx(cx, |me, cx| me.child.rebuild(cx, element))
    }
}
