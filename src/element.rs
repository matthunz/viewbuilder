use concoct::{Object, Slot};

pub struct Element<V> {
    view: V,
}

impl<V> Object for Element<V> {}

pub struct LayoutMessage;

impl<V> Slot<LayoutMessage> for Element<V> {
    fn handle(&mut self, _handle: concoct::Context<Self>, _msg: LayoutMessage) {
        todo!()
    }
}
