use std::sync::Arc;

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
