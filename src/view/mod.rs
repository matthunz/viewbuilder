use crate::Context;
use std::{marker::PhantomData, sync::Arc};

mod from_fn;
pub use self::from_fn::{from_fn, FromFn};

mod map;
pub use self::map::Map;

pub trait View<M> {
    type Element;

    fn build(&mut self, cx: &mut Context<M>) -> Self::Element;

    fn rebuild(&mut self, cx: &mut Context<M>, element: &mut Self::Element);

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

impl<M> View<M> for () {
    type Element = ();

    fn build(&mut self, _cx: &mut Context<M>) -> Self::Element {}

    fn rebuild(&mut self, _cx: &mut Context<M>, _element: &mut Self::Element) {}
}

macro_rules! impl_viewbuilder_for_tuple {
    ($($t:tt: $idx:tt),*) => {
        impl<M, $($t: View<M>),*> View<M> for ($($t),*) {
            type Element = ($($t::Element),*);

            fn build(&mut self, cx: &mut Context<M>) -> Self::Element {
                ($(self.$idx.build(cx)),*)
            }

            fn rebuild(&mut self, cx: &mut Context<M>, element: &mut Self::Element) {
                $(self.$idx.rebuild(cx, &mut element.$idx));*
            }
        }
    };
}

impl_viewbuilder_for_tuple!(V1: 0, V2: 1);
impl_viewbuilder_for_tuple!(V1: 0, V2: 1, V3: 2);
impl_viewbuilder_for_tuple!(V1: 0, V2: 1, V3: 2, V4: 3);
impl_viewbuilder_for_tuple!(V1: 0, V2: 1, V3: 2, V4: 3, V5: 4);
impl_viewbuilder_for_tuple!(V1: 0, V2: 1, V3: 2, V4: 3, V5: 4, V6: 5);
impl_viewbuilder_for_tuple!(V1: 0, V2: 1, V3: 2, V4: 3, V5: 4, V6: 5, V7: 6);
impl_viewbuilder_for_tuple!(V1: 0, V2: 1, V3: 2, V4: 3, V5: 4, V6: 5, V7: 6, V8: 7);
