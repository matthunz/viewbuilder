use bumpalo::Bump;
use std::mem;

pub trait View<'a, M> {
    type Element;

    fn build(&'a mut self) -> Self::Element;

    fn rebuild(&'a mut self, element: &mut Self::Element);

    fn handle(&'a mut self, msg: M);
}

pub struct Tree<V, S, F> {
    component: fn(&Bump, &mut S) -> V,
    state: S,
    handler: F,
    frame_a: Bump,
    frame_b: Bump,
    is_frame_a: bool,
}

impl<V, S, F> Tree<V, S, F> {
    pub fn new<'a, M>(state: S, component: fn(&'a Bump, &mut S) -> V, handler: F) -> Self
    where
        V: View<'a, M> + 'a,
    {
        let component = unsafe { mem::transmute(component) };
        Self {
            component,
            handler,
            state,
            frame_a: Bump::new(),
            frame_b: Bump::new(),
            is_frame_a: true,
        }
    }

    pub fn view<'a, M>(&mut self)
    where
        V: View<'a, M> + 'a,
    {
        let bump = if self.is_frame_a {
            self.is_frame_a = false;
            &self.frame_a
        } else {
            self.is_frame_a = true;
            &self.frame_b
        };
        let bump: &'a Bump = unsafe { mem::transmute(bump) };

        let view = (self.component)(bump, &mut self.state);
        bump.alloc(view).build();
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
