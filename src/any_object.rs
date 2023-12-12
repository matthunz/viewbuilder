use crate::{HandleState, Object};
use slotmap::DefaultKey;
use std::any::Any;

/// A dynamic reactive object.
pub trait AnyObject {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn start_any(&mut self, key: DefaultKey);
}

impl<O> AnyObject for O
where
    O: Object + 'static,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn start_any(&mut self, key: DefaultKey) {
        self.start(crate::Handle {
            state: HandleState::new(key),
            handle: HandleState::new(key).into(),
        })
    }
}
