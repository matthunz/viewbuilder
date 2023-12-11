use crate::{Handle, UserInterface};
use kurbo::Size;

mod text;
pub use self::text::{Text, TextMessage};

mod linear_layout;
pub use self::linear_layout::LinearLayout;

mod window;
pub use self::window::Window;

pub trait Element {
    type Message;

    fn update(&mut self, cx: Handle<Self>, msg: Self::Message);

    fn layout(&mut self, min_size: Option<Size>, max_size: Option<Size>) -> Size;

    fn spawn(self) -> Handle<Self>
    where
        Self: Sized + 'static,
    {
        UserInterface::current().insert(self)
    }
}
