use crate::Context;
use std::any::Any;
use std::mem;
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

    fn remove(&mut self, cx: &mut Context<M>, state: &mut T, element: Self::Element);

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

    fn remove(&mut self, _cx: &mut Context<M>, _state: &mut T, _element: Self::Element) {}
}

impl<T, M, V: View<T, M>> View<T, M> for Option<V> {
    type Element = Option<(V, V::Element)>;

    fn build(&mut self, cx: &mut Context<M>, state: &mut T) -> Self::Element {
        self.take().map(|mut view| {
            let element = view.build(cx, state);
            (view, element)
        })
    }

    fn rebuild(&mut self, cx: &mut Context<M>, state: &mut T, element: &mut Self::Element) {
        if let Some(mut view) = self.take() {
            if let Some((_, element)) = element {
                view.rebuild(cx, state, element);
            } else {
                let elem = view.build(cx, state);
                *element = Some((view, elem));
            }
        } else if let Some((mut view, elem)) = element.take() {
            view.remove(cx, state, elem);
        }
    }

    fn remove(&mut self, cx: &mut Context<M>, state: &mut T, element: Self::Element) {
        if let Some((mut view, element)) = element {
            view.remove(cx, state, element)
        }
    }
}

pub trait AnyView<T, M> {
    fn build_any(&mut self, cx: &mut Context<M>, state: &mut T) -> Box<dyn Any>;

    fn rebuild_any(&mut self, cx: &mut Context<M>, state: &mut T, element: &mut Box<dyn Any>);

    fn remove_any(&mut self, cx: &mut Context<M>, state: &mut T, element: Box<dyn Any>);
}

impl<T, M, V> AnyView<T, M> for V
where
    V: View<T, M>,
    V::Element: 'static,
{
    fn build_any(&mut self, cx: &mut Context<M>, state: &mut T) -> Box<dyn Any> {
        Box::new((&mut *self).build(cx, state))
    }

    fn rebuild_any(&mut self, cx: &mut Context<M>, state: &mut T, element: &mut Box<dyn Any>) {
        if element.is::<V::Element>() {
            (&mut *self).rebuild(cx, state, element.downcast_mut().unwrap())
        } else {
            *element = self.build_any(cx, state);
        }
    }

    fn remove_any(&mut self, cx: &mut Context<M>, state: &mut T, element: Box<dyn Any>) {
        (&mut *self).remove(cx, state, *element.downcast().unwrap())
    }
}

impl<T, M> View<T, M> for Box<dyn AnyView<T, M>> {
    type Element = Box<dyn Any>;

    fn build(&mut self, cx: &mut Context<M>, state: &mut T) -> Self::Element {
        (&mut **self).build_any(cx, state)
    }

    fn rebuild(&mut self, cx: &mut Context<M>, state: &mut T, element: &mut Self::Element) {
        (&mut **self).rebuild_any(cx, state, element)
    }

    fn remove(&mut self, cx: &mut Context<M>, state: &mut T, element: Self::Element) {
        (&mut **self).remove_any(cx, state, element)
    }
}

macro_rules! generate_on_of {
    ($id:ident, $data_id:ident, $($name:tt: $t:tt),*) => {
        pub struct $id<$($t),*> {
            data: Option<$data_id<$($t),*>>,
        }

        impl<$($t),*> $id<$($t),*> {
            $(
                pub fn $name(value: $t) -> Self {
                    Self {
                        data: Some($data_id::$t(value)),
                    }
                }
            )*

            pub fn data(&self) -> &$data_id<$($t),*> {
                self.data.as_ref().unwrap()
            }
        }

        pub enum $data_id<$($t),*> {
            $($t($t)),*
        }

        impl<ViewT, ViewM, $($t: View<ViewT, ViewM>),*> View<ViewT, ViewM> for $id<$($t),*> {
            type Element = $data_id<$(($t, $t::Element)),*>;

            fn build(&mut self, cx: &mut Context<ViewM>, state: &mut ViewT) -> Self::Element {
                match self.data.take().unwrap() {
                    $(
                        $data_id::$t(mut view) => {
                            let elem = view.build(cx, state);
                            $data_id::$t((view, elem))
                        },
                    )*
                }

            }

            fn rebuild(&mut self, cx: &mut Context<ViewM>, state: &mut ViewT, element: &mut Self::Element) {
                match (self.data.take().unwrap(), element) {
                    $(
                        ($data_id::$t(mut view), $data_id::$t((last_view, last_elem))) => {
                            view.rebuild(cx, state, last_elem);
                            *last_view = view
                        }
                    )*
                    (data, element) => {
                        let elem = mem::replace(element, Self { data: Some(data) }.build(cx, state));
                        match elem {
                            $(
                                $data_id::$t((mut view, element)) => {
                                    view.remove(cx, state, element);
                                }
                            )*
                        }
                    }
                }
            }

            fn remove(&mut self, cx: &mut Context<ViewM>, state: &mut ViewT, element: Self::Element) {
                match element {
                    $(
                        $data_id::$t((mut view, elem)) => view.remove(cx, state, elem),
                    )*
                }
            }
        }
    };
}

generate_on_of!(OneOf2, OneOf2Data, a: A, b: B);
generate_on_of!(OneOf3, OneOf3Data, a: A, b: B, c: C);
generate_on_of!(OneOf4, OneOf4Data, a: A, b: B, c: C, d: D);
generate_on_of!(OneOf5, OneOf5Data, a: A, b: B, c: C, d: D, e: E);
generate_on_of!(OneOf6, OneOf6Data, a: A, b: B, c: C, d: D, e: E, f: F);
generate_on_of!(OneOf7, OneOf7Data, a: A, b: B, c: C, d: D, e: E, f: F, g: G);
generate_on_of!(OneOf8, OneOf8Data, a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H);

pub struct OneOf<A, B> {
    data: Option<OneOfData<A, B>>,
}

impl<A, B> OneOf<A, B> {
    pub fn a(value: A) -> Self {
        Self {
            data: Some(OneOfData::A(value)),
        }
    }

    pub fn b(value: B) -> Self {
        Self {
            data: Some(OneOfData::B(value)),
        }
    }

    pub fn data(&self) -> &OneOfData<A, B> {
        self.data.as_ref().unwrap()
    }
}

pub enum OneOfData<A, B> {
    A(A),
    B(B),
}

impl<ViewT, ViewM, A, B> View<ViewT, ViewM> for OneOf<A, B>
where
    A: View<ViewT, ViewM>,
    B: View<ViewT, ViewM>,
{
    type Element = OneOfData<(A, A::Element), (B, B::Element)>;

    fn build(&mut self, cx: &mut Context<ViewM>, state: &mut ViewT) -> Self::Element {
        match self.data.take().unwrap() {
            OneOfData::A(mut view) => {
                let elem = view.build(cx, state);
                OneOfData::A((view, elem))
            }
            OneOfData::B(mut view) => {
                let elem = view.build(cx, state);
                OneOfData::B((view, elem))
            }
        }
    }

    fn rebuild(&mut self, cx: &mut Context<ViewM>, state: &mut ViewT, element: &mut Self::Element) {
        match self.data.take().unwrap() {
            OneOfData::A(mut view) => match element {
                OneOfData::A((last_view, elem)) => {
                    view.rebuild(cx, state, elem);
                    *last_view = view;
                }
                OneOfData::B(_) => {
                    let elem = view.build(cx, state);

                    let last_element = mem::replace(element, OneOfData::A((view, elem)));
                    match last_element {
                        OneOfData::B((mut last_view, last_elem)) => {
                            last_view.remove(cx, state, last_elem)
                        }
                        _ => unimplemented!(),
                    }
                }
            },
            OneOfData::B(mut view) => match element {
                OneOfData::A(_) => {
                    let elem = view.build(cx, state);

                    let last_element = mem::replace(element, OneOfData::B((view, elem)));
                    match last_element {
                        OneOfData::A((mut last_view, last_elem)) => {
                            last_view.remove(cx, state, last_elem)
                        }
                        _ => unimplemented!(),
                    }
                }
                OneOfData::B((last_view, elem)) => {
                    view.rebuild(cx, state, elem);
                    *last_view = view;
                }
            },
        }
    }

    fn remove(&mut self, cx: &mut Context<ViewM>, state: &mut ViewT, element: Self::Element) {
        match element {
            OneOfData::A((mut view, elem)) => view.remove(cx, state, elem),
            OneOfData::B((mut view, elem)) => view.remove(cx, state, elem),
        }
    }
}

impl<T, M, V, K> View<T, M> for Vec<(K, V)>
where
    K: PartialEq + Clone,
    V: View<T, M>,
{
    type Element = Vec<(K, V, V::Element)>;

    fn build(&mut self, cx: &mut Context<M>, tree: &mut T) -> Self::Element {
        self.drain(..)
            .map(|(key, mut view)| {
                let elem = view.build(cx, tree);
                (key, view, elem)
            })
            .collect()
    }

    fn rebuild(&mut self, cx: &mut Context<M>, state: &mut T, element: &mut Self::Element) {
        let mut visited = Vec::new();
        for (idx, (key, mut view)) in self.drain(..).enumerate() {
            if let Some((_key, _last_view, element)) = element
                .iter_mut()
                .find(|(view_key, _view, _element)| view_key == &key)
            {
                view.rebuild(cx, state, element)
            } else {
                let elem = view.build(cx, state);
                element.insert(idx, (key.clone(), view, elem))
            }
            visited.push(key);
        }

        let mut removes = Vec::new();
        for (idx, (key, _view, _elem)) in element.iter_mut().enumerate() {
            if !visited.contains(key) {
                removes.push(idx);
            }
        }
        for idx in removes {
            element.remove(idx);
        }
    }

    fn remove(&mut self, cx: &mut Context<M>, state: &mut T, element: Self::Element) {
        for (_key, mut view, elem) in element {
            view.remove(cx, state, elem);
        }
    }
}

macro_rules! impl_view_for_tuple {
    ($($t:tt: $idx:tt),*) => {
        impl<T, M, $($t: View<T, M>),*> View<T, M> for ($($t),*) {
            type Element = ($($t::Element),*);

            fn build(&mut self, cx: &mut Context<M>, tree: &mut T) -> Self::Element {
                #[cfg(feature = "tracing")]
                let name = stringify!(($($t),*));
                #[cfg(feature = "tracing")]
                crate::build_span!(name);

                ($(self.$idx.build(cx, tree)),*)
            }

            fn rebuild(&mut self, cx: &mut Context<M>, tree: &mut T, element: &mut Self::Element) {
                #[cfg(feature = "tracing")]
                let name = stringify!(($($t),*));
                #[cfg(feature = "tracing")]
                crate::rebuild_span!(name);

                $(self.$idx.rebuild(cx, tree, &mut element.$idx));*
            }

            fn remove(&mut self, cx: &mut Context<M>, tree: &mut T, element: Self::Element) {
                #[cfg(feature = "tracing")]
                let name = stringify!(($($t),*));
                #[cfg(feature = "tracing")]
                crate::remove_span!(name);

                $(self.$idx.remove(cx, tree, element.$idx));*
            }
        }
    };
}

impl_view_for_tuple!(V1: 0, V2: 1);
impl_view_for_tuple!(V1: 0, V2: 1, V3: 2);
impl_view_for_tuple!(V1: 0, V2: 1, V3: 2, V4: 3);
impl_view_for_tuple!(V1: 0, V2: 1, V3: 2, V4: 3, V5: 4);
impl_view_for_tuple!(V1: 0, V2: 1, V3: 2, V4: 3, V5: 4, V6: 5);
impl_view_for_tuple!(V1: 0, V2: 1, V3: 2, V4: 3, V5: 4, V6: 5, V7: 6);
impl_view_for_tuple!(V1: 0, V2: 1, V3: 2, V4: 3, V5: 4, V6: 5, V7: 6, V8: 7);
