use crate::{render, Component, Element, View, AnyElement};
use bumpalo::Bump;
use std::mem;

pub struct App<C> {
    component: C,
    element: Option<Box<dyn AnyElement>>,
    bump: Bump,
}

impl<C> App<C> {
    pub fn new(component: C) -> Self
    where
        C: Component,
    {
        let component = unsafe { mem::transmute(component) };
        Self {
            component,
            element: None,
            bump: Bump::new(),
        }
    }

    pub fn view(&mut self)
    where
        C: Component,
    {
        let mut view = self.component.view(&self.bump);
        let view = self.bump.alloc(view);

        if let Some(ref mut element) = self.element {
            view.rebuild(element.as_any_mut().downcast_mut().unwrap());
        } else {
            self.element = Some(Box::new(view.build()));
        }
    }

    pub fn handle<'a>(&mut self, msg: C::Message)
    where
        C: Component,
    {
        self.component.update(msg);
    }

    pub fn run(&mut self)
    where
        C: Component,
    {
        self.view();

        render::run(self.element.as_mut().unwrap().as_element_mut());
    }
}
