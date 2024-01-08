use crate::Context;
use std::{marker::PhantomData, sync::Arc};

mod from_fn;
pub use self::from_fn::{from_fn, FromFn};

mod lazy;
pub use self::lazy::{lazy, Lazy};

mod map;
pub use self::map::Map;

mod once;
pub use self::once::{once, Once};

pub trait View<T, M> {
    type Element;

    fn build(&mut self, cx: &mut Context<M>, state: &mut T) -> Self::Element;

    fn rebuild(&mut self, cx: &mut Context<M>, state: &mut T, element: &mut Self::Element);

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

impl<T, M> View<T, M> for () {
    type Element = ();

    fn build(&mut self, _cx: &mut Context<M>, _tree: &mut T) -> Self::Element {}

    fn rebuild(&mut self, _cx: &mut Context<M>, _tree: &mut T, _element: &mut Self::Element) {}
}

impl<T, M, V, K> View<T, M> for Vec<(K, V)>
where
    K: PartialEq,
    V: View<T, M>,
{
    type Element = Vec<V::Element>;

    fn build(&mut self, cx: &mut Context<M>, tree: &mut T) -> Self::Element {
        self.iter_mut()
            .map(|(_key, view)| view.build(cx, tree))
            .collect()
    }

    fn rebuild(&mut self, _cx: &mut Context<M>, _tree: &mut T, _element: &mut Self::Element) {
        todo!()
    }
}

macro_rules! impl_viewbuilder_for_tuple {
    ($($t:tt: $idx:tt),*) => {
        impl<T, M, $($t: View<T, M>),*> View<T, M> for ($($t),*) {
            type Element = ($($t::Element),*);

            fn build(&mut self, cx: &mut Context<M>, tree: &mut T) -> Self::Element {
                ($(self.$idx.build(cx, tree)),*)
            }

            fn rebuild(&mut self, cx: &mut Context<M>, tree: &mut T, element: &mut Self::Element) {
                $(self.$idx.rebuild(cx, tree, &mut element.$idx));*
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
