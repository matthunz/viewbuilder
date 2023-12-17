use crate::Context;
use std::{marker::PhantomData, sync::Arc};

mod map;
pub use self::map::Map;

pub trait ViewBuilder<M> {
    type View;

    fn build(&mut self, cx: &mut Context<M>) -> Self::View;

    fn rebuild(&mut self, cx: &mut Context<M>, state: &mut Self::View);

    fn map<F, M1>(self, f: F) -> Map<Self, F, M>
    where
        Self: Sized,
        F: Fn(M) -> M1 + 'static,
        M1: 'static,
    {
        Map {
            view: self,
            f: Arc::new(f),
            _marker: PhantomData,
        }
    }
}
