mod any_element;

mod element_ref;
pub use element_ref::ElementRef;

mod transaction;
pub use transaction::Transaction;

mod user_interface;
pub use user_interface::UserInterface;

mod view;
pub use view::View;

pub trait Element {}

impl Element for () {}
