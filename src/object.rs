use crate::{Handle, HandleState, Node, Runtime};
use std::{cell::RefCell, marker::PhantomData, rc::Rc};

/// A reactive object.
pub trait Object: Sized {
    /// Update event sender for this object.
    type Sender: From<HandleState<Self>> + Clone;

    /// Spawn this object and return a handle to it.
    fn spawn(self) -> Handle<Self>
    where
        Self: 'static,
    {
        let key = Runtime::current().inner.borrow_mut().nodes.insert(Node {
            object: Rc::new(RefCell::new(self)),
            listeners: Vec::new(),
        });

        Handle {
            state: HandleState {
                key,
                _marker: PhantomData,
            },
            sender: HandleState {
                key,
                _marker: PhantomData,
            }
            .into(),
        }
    }
}
