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

pub struct Application<T, F, S, M, Tree> {
    model: T,
    composable: F,
    state: Option<S>,
    cx: Context<M>,
    tree: Tree,
}

impl<T, F, S, M, Tree> Application<T, F, S, M, Tree> {
    pub fn new(send: Arc<dyn Fn(M)>, model: T, composable: F, tree: Tree) -> Self
    where
        M: Send + 'static,
    {
        let cx = Context::new(send);

        Self {
            model,
            composable,
            state: None,
            cx,

            tree,
        }
    }

    pub fn build<C>(&mut self)
    where
        T: Model<M>,
        F: FnMut(&T) -> C,
        C: View<Tree, M, Element = S>,
    {
        let state = (self.composable)(&self.model).build(&mut self.cx, &mut self.tree);
        self.state = Some(state);
    }

    pub fn rebuild<C>(&mut self)
    where
        T: Model<M>,
        F: FnMut(&T) -> C,
        C: View<Tree, M, Element = S>,
    {
        let state = self.state.as_mut().unwrap();
        (self.composable)(&self.model).rebuild(&mut self.cx, &mut self.tree, state);
    }

    pub fn handle(&mut self, msg: M) -> ControlFlow
    where
        T: Model<M>,
        M: 'static,
    {
        self.model.handle(msg)
    }
}
