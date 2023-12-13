use concoct::{Handle, Object, Slot, Signal};
use web_sys::wasm_bindgen::JsCast;

pub trait View {
    fn node(&self) -> &web_sys::Node;

    fn set_parent(&mut self, parent: web_sys::Element);
}

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

impl<A: IntoView, B: IntoView, C: IntoView> ViewGroup for (A, B, C) {
    type Handles = (Handle<A::View>, Handle<B::View>, Handle<C::View>);

    fn view_group(self) -> Self::Handles {
        (self.0.into_view(), self.1.into_view(), self.2.into_view())
    }
}

pub struct MouseEvent;

pub struct ElementBuilder {}

impl ElementBuilder {
   

    pub fn child(&mut self, _view: impl ViewGroup) -> &mut Self {
        self
    }

    pub fn build(&mut self) -> Element {
        todo!()
    }
}

pub struct Element {
    pub element: web_sys::Element,
    parent: Option<web_sys::Element>,
}

impl Element {
    pub fn new(tag: &str) -> Self {
        Self {
            element: web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .create_element(tag)
                .unwrap(),
            parent: None,
        }
    }

    pub fn builder() -> ElementBuilder {
        ElementBuilder {}
    }
}

impl Object for Element {}

impl View for Element {
    fn node(&self) -> &web_sys::Node {
        self.element.unchecked_ref()
    }

    fn set_parent(&mut self, parent: web_sys::Element) {
        self.parent = Some(parent.clone());
        parent.append_child(&self.element).unwrap();
    }
}

impl Signal<MouseEvent> for Element {

}

pub struct AppendChild<V>(pub Handle<V>);

impl<V: View + 'static> Slot<AppendChild<V>> for Element {
    fn handle(&mut self, _handle: concoct::Handle<Self>, msg: AppendChild<V>) {
        self.element.append_child(msg.0.borrow().node()).unwrap();
    }
}

pub struct Text {
    pub node: web_sys::Text,
    parent: Option<web_sys::Element>,
}

impl Text {
    pub fn new(content: &str) -> Self {
        Self {
            node: web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .create_text_node(content),
            parent: None,
        }
    }
}

impl View for Text {
    fn node(&self) -> &web_sys::Node {
        &self.node
    }

    fn set_parent(&mut self, parent: web_sys::Element) {
        self.parent = Some(parent.clone());
        parent.append_child(&self.node).unwrap();
    }
}

impl Object for Text {}

impl Slot<String> for Text {
    fn handle(&mut self, _handle: concoct::Handle<Self>, _msg: String) {}
}
