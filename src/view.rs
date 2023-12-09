use crate::{view, Element, ElementRef};
use slotmap::DefaultKey;

pub trait View {
    fn view(self) -> DefaultKey;
}

impl<E: Element + 'static> View for E {
    fn view(self) -> DefaultKey {
        view(self).key
    }
}

impl<E> View for ElementRef<E> {
    fn view(self) -> DefaultKey {
        self.key
    }
}

impl View for DefaultKey {
    fn view(self) -> DefaultKey {
        self
    }
}
