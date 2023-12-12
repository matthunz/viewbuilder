use concoct::Handle;

mod linear_layout;
pub use self::linear_layout::LinearLayout;

mod view_group;
pub use self::view_group::ViewGroup;

mod text;
pub use self::text::Text;

pub trait View {
    fn view(self);
}

impl<V> View for Handle<V> {
    fn view(self) {
        todo!()
    }
}
