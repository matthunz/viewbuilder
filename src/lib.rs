use bumpalo::Bump;
use slotmap::{DefaultKey, SlotMap};
use std::{any::Any, borrow::Cow, mem};

pub use bumpalo::collections::String as BumpString;

pub trait View<'a, M> {
    type Element;

    fn build(&'a mut self) -> Self::Element;

    fn rebuild(&'a mut self, element: &mut Self::Element);

    fn handle(&'a mut self, msg: M);
}

pub struct Tree<V, S, E, F> {
    component: fn(&Bump, &mut S) -> V,
    state: S,
    element: Option<E>,
    handler: F,
    bump: Bump,
}

impl<V, S, E, F> Tree<V, S, E, F> {
    pub fn new<'a, M>(state: S, component: fn(&'a Bump, &mut S) -> V, handler: F) -> Self
    where
        V: View<'a, M, Element = E> + 'a,
    {
        let component = unsafe { mem::transmute(component) };
        Self {
            component,
            handler,
            state,
            element: None,
            bump: Bump::new(),
        }
    }

    pub fn view<'a, M>(&mut self)
    where
        V: View<'a, M, Element = E> + 'a,
    {
        let bump: &'a Bump = unsafe { mem::transmute(&self.bump) };

        let view = bump.alloc((self.component)(bump, &mut self.state));
        if let Some(ref mut element) = self.element {
            view.rebuild(element);
        } else {
            self.element = Some(view.build());
        }
    }

    pub fn handle<'a, M>(&mut self, msg: M)
    where
        V: View<'a, M> + 'a,
        F: FnMut(&mut S, M),
    {
        (self.handler)(&mut self.state, msg);
    }
}

impl<'a, M> View<'a, M> for &'a str {
    type Element = ();

    fn build(&'a mut self) -> Self::Element {
        dbg!(self);
    }

    fn rebuild(&'a mut self, _element: &mut Self::Element) {
        dbg!(self);
    }

    fn handle(&'a mut self, _msg: M) {}
}

#[macro_export]
macro_rules! format_in {
    ($bump:expr, $($arg:tt)*) => {
        {
            use std::fmt::Write;

            let mut s = viewbuilder::BumpString::new_in($bump);
            write!(&mut s, $($arg)*).unwrap();

            // TODO
            &**$bump.alloc(s)
        }
    };
}

pub struct Text<'a, M> {
    content: Cow<'a, str>,
    on_click: Option<M>,
}

impl<'a, M> Text<'a, M> {
    pub fn new(content: impl Into<Cow<'a, str>>) -> Self {
        Self {
            content: content.into(),
            on_click: None,
        }
    }

    pub fn on_click(mut self, msg: M) -> Self {
        self.on_click = Some(msg);
        self
    }
}

impl<'a, M> View<'a, M> for Text<'a, M> {
    type Element = ();

    fn build(&'a mut self) -> Self::Element {
        todo!()
    }

    fn rebuild(&'a mut self, element: &mut Self::Element) {
        todo!()
    }

    fn handle(&'a mut self, msg: M) {
        todo!()
    }
}

pub struct Flex<V> {
    view: V,
}

impl<V> Flex<V> {
    pub fn new(view: V) -> Self {
        Self { view }
    }
}

impl<'a, M, V> View<'a, M> for Flex<V> {
    type Element = ();

    fn build(&'a mut self) -> Self::Element {
        todo!()
    }

    fn rebuild(&'a mut self, element: &mut Self::Element) {
        todo!()
    }

    fn handle(&'a mut self, msg: M) {
        todo!()
    }
}
