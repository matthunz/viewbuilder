pub use bumpalo::collections::String as BumpString;
use bumpalo::Bump;

mod app;
pub use app::App;

mod any_element;
pub use any_element::AnyElement;

mod element;
pub use element::Element;

mod view;
pub use view::{LinearLayout, Text, View};

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

pub fn run<'a, S, V, F, E, M>(state: S, component: fn(&'a Bump, &mut S) -> V, handler: F)
where
    V: View<'a, M, Element = E> + 'a,
    E: Element,
    F: FnMut(&mut S, M),
{
    let mut app = App::new(state, component, handler);
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
