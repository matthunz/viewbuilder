use concoct::{Handle, Object};

mod linear_layout;
pub use self::linear_layout::LinearLayout;

mod view_group;
pub use self::view_group::ViewGroup;

mod text;
pub use self::text::Text;

pub trait View {}

pub trait IntoView {
    type View: View;

    fn into_view(self) -> Handle<Self::View>;
}

impl<V> IntoView for V
where
    V: View + Object + 'static,
{
    type View = V;

    fn into_view(self) -> Handle<Self::View> {
        self.start()
    }
}

impl<V: View> IntoView for Handle<V> {
    type View = V;

    fn into_view(self) -> Handle<Self::View> {
        self
    }
}
