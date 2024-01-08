use crate::{Context, ControlFlow, Model, View};

/// Runtime for a model and view builder.
pub struct Runtime<T, VB, E, M, S> {
    model: T,
    view_builder: VB,
    element: Option<E>,
    cx: Context<M>,
    state: S,
}

impl<T, VB, E, M, S> Runtime<T, VB, E, M, S> {
    /// Create a new runtime.
    ///
    /// The send function will receive messages from the runtime's [`Context`].
    pub fn new(send: impl Fn(M) + 'static, model: T, view_builder: VB, state: S) -> Self
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

    /// Build the view.
    pub fn build<V>(&mut self)
    where
        T: Model<M>,
        VB: FnMut(&T) -> V,
        V: View<S, M, Element = E>,
    {
        let state = (self.view_builder)(&self.model).build(&mut self.cx, &mut self.state);
        self.element = Some(state);
    }

    /// Rebuild the view.
    pub fn rebuild<V>(&mut self)
    where
        T: Model<M>,
        VB: FnMut(&T) -> V,
        V: View<S, M, Element = E>,
    {
        let state = self.element.as_mut().unwrap();
        (self.view_builder)(&self.model).rebuild(&mut self.cx, &mut self.state, state);
    }

    /// Send a message to the model.
    pub fn handle(&mut self, msg: M) -> ControlFlow
    where
        T: Model<M>,
        M: 'static,
    {
        self.model.handle(msg)
    }
}
