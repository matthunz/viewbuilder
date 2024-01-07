use std::sync::Arc;
use tokio::sync::mpsc;

pub mod view;
pub use self::view::View;

pub struct Context<M> {
    send: Arc<dyn Fn(M) + Send + Sync>,
}

impl<M> Context<M> {
    pub fn new(send: Arc<dyn Fn(M) + Send + Sync>) -> Self {
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

pub struct Tree<T, F, S, M> {
    model: T,
    composable: F,
    state: Option<S>,
    cx: Context<M>,
    rx: mpsc::UnboundedReceiver<M>,
}

impl<T, F, S, M> Tree<T, F, S, M> {
    pub fn new(model: T, composable: F) -> Self
    where
        M: Send + 'static,
    {
        let (tx, rx) = mpsc::unbounded_channel();
        let cx = Context::new(Arc::new(move |msg| {
            tx.send(msg).unwrap();
        }));
        Self {
            model,
            composable,
            state: None,
            cx,
            rx,
        }
    }

    pub fn build<C>(&mut self)
    where
        T: Model<M>,
        F: FnMut(&T) -> C,
        C: View<M, Element = S>,
    {
        let state = (self.composable)(&self.model).build(&mut self.cx);
        self.state = Some(state);
    }

    pub fn rebuild<C>(&mut self)
    where
        T: Model<M>,
        F: FnMut(&T) -> C,
        C: View<M, Element = S>,
    {
        let state = self.state.as_mut().unwrap();
        (self.composable)(&self.model).rebuild(&mut self.cx, state);
    }

    pub async fn handle(&mut self) -> ControlFlow
    where
        T: Model<M>,
        M: 'static,
    {
        let msg = self.rx.recv().await.unwrap();
        self.model.handle(msg)
    }

    pub fn try_handle(&mut self) -> Option<ControlFlow>
    where
        T: Model<M>,
        M: 'static,
    {
        if let Ok(msg) = self.rx.try_recv() {
            Some(self.model.handle(msg))
        } else {
            None
        }
    }

    pub fn try_run<C>(&mut self)
    where
        T: Model<M>,
        F: FnMut(&T) -> C,
        C: View<M, Element = S>,
        M: 'static,
    {
        match self.try_handle() {
            Some(ControlFlow::Rebuild) => self.rebuild(),
            Some(ControlFlow::Pending) | None => {}
        }
    }

    pub async fn run<C>(&mut self)
    where
        T: Model<M>,
        F: FnMut(&T) -> C,
        C: View<M, Element = S>,
        M: 'static,
    {
        match self.handle().await {
            ControlFlow::Rebuild => self.rebuild(),
            ControlFlow::Pending => {}
        }
    }
}
