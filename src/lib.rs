pub use bumpalo::collections::String as BumpString;
use bumpalo::Bump;

mod app;
pub use app::App;

mod any_element;
pub use any_element::AnyElement;

mod element;
pub use element::Element;

mod view;
use kurbo::Point;

pub use view::{Direction, LinearLayout, Text, View};

mod view_group;
pub use view_group::ViewGroup;

mod render;

#[macro_export]
macro_rules! format_in {
    ($bump:expr, $($arg:tt)*) => {
        {
            use std::fmt::Write;

            let mut s = viewbuilder::BumpString::new_in($bump);
            write!(&mut s, $($arg)*).unwrap();

            // TODO
            &**$bump.alloc(s)
        }
    };
}

pub trait Component {
    type Message: 'static;

    fn update(&mut self, msg: Self::Message);

    fn view<'a>(&mut self, bump: &'a Bump) -> impl View<'a, Self::Message>;
}

pub fn run(component: impl Component) {
    let mut app = App::new(component);
    app.run();
}

pub struct Node {
    element: Box<dyn AnyElement>,
}

impl Node {
    pub fn new(element: impl Element + 'static) -> Self {
        Self {
            element: Box::new(element),
        }
    }
}

#[derive(Clone)]
pub enum WindowMessage {
    Click { position: Point },
}
