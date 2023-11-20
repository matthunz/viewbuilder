use crate::View;
use bumpalo::Bump;
use std::mem;

pub struct App<V, S, E, F> {
    component: fn(&Bump, &mut S) -> V,
    state: S,
    element: Option<E>,
    handler: F,
    bump: Bump,
}

impl<V, S, E, F> App<V, S, E, F> {
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
