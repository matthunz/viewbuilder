use concoct::Handle;

use super::IntoView;

pub trait ViewGroup {
    type Handles;

    fn view_group(self) -> Self::Handles;
}

impl<V: IntoView> ViewGroup for V {
    type Handles = Handle<V::View>;

    fn view_group(self) -> Self::Handles {
        self.into_view()
    }
}

impl<A: IntoView, B: IntoView> ViewGroup for (A, B) {
    type Handles = (Handle<A::View>, Handle<B::View>);

    fn view_group(self) -> Self::Handles {
        (self.0.into_view(), self.1.into_view())
    }
}
