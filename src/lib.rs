use std::sync::Arc;

pub mod view;
pub use self::view::View;

pub mod web;

pub struct Context<M> {
    send: Arc<dyn Fn(M)>,
}

impl<M> Context<M> {
    pub fn new(send: Arc<dyn Fn(M)>) -> Self {
        Self { send }
    }

    pub fn send(&self, msg: M) {
        (self.send)(msg)
    }
}

impl<M> Clone for Context<M> {
    fn clone(&self) -> Self {
        Self {
            send: self.send.clone(),
        }
    }
}

pub enum ControlFlow {
    Pending,
    Rebuild,
}

pub trait Model<M> {
    fn handle(&mut self, msg: M) -> ControlFlow;
}

pub struct Runtime<T, VB, E, M, S> {
    model: T,
    view_builder: VB,
    element: Option<E>,
    cx: Context<M>,
    state: S,
}

impl<T, VB, E, M, S> Runtime<T, VB, E, M, S> {
    pub fn new(send: Arc<dyn Fn(M)>, model: T, view_builder: VB, state: S) -> Self
    where
        M: Send + 'static,
    {
        let cx = Context::new(send);

        Self {
            model,
            view_builder,
            element: None,
            cx,
            state,
        }
    }

    pub fn build<V>(&mut self)
    where
        T: Model<M>,
        VB: FnMut(&T) -> V,
        V: View<S, M, Element = E>,
    {
        let state = (self.view_builder)(&self.model).build(&mut self.cx, &mut self.state);
        self.element = Some(state);
    }

    pub fn rebuild<V>(&mut self)
    where
        T: Model<M>,
        VB: FnMut(&T) -> V,
        V: View<S, M, Element = E>,
    {
        let state = self.element.as_mut().unwrap();
        (self.view_builder)(&self.model).rebuild(&mut self.cx, &mut self.state, state);
    }

    pub fn handle(&mut self, msg: M) -> ControlFlow
    where
        T: Model<M>,
        M: 'static,
    {
        self.model.handle(msg)
    }
}
