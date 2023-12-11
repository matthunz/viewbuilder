use std::{cell::RefCell, rc::Rc};

mod any_element;
pub use self::any_element::AnyElement;

pub mod element;
pub use self::element::Element;

mod handle;
pub use self::handle::Handle;

mod ui;
pub use self::ui::UserInterface;

pub struct Node {
    element: Rc<RefCell<dyn AnyElement>>,
}
