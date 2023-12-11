use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use crate::{Handle, HandleState, Node, Runtime};
pub trait Object: Sized {
    type Sender: From<HandleState<Self>> + Clone;

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
