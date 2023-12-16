use concoct::{Binding, Context, Handle, Object, Signal};
use js_sys::wasm_bindgen::closure::Closure;
use web_sys::wasm_bindgen::JsCast;

pub struct Element {
    raw: web_sys::Element,
    children: Vec<Handle<Self>>,
}

impl Element {
    pub fn new(tag: &str) -> Self {
        let document = web_sys::window().unwrap().document().unwrap();
        let raw = document.create_element(tag).unwrap();
        Self::from_raw(raw)
    }

    pub fn body() -> Self {
        let document = web_sys::window().unwrap().document().unwrap();
        let body = document.body().unwrap().unchecked_into();
        Self::from_raw(body)
    }

    pub fn from_raw(raw: web_sys::Element) -> Self {
        Self {
            raw,
            children: Vec::new(),
        }
    }

    pub fn set_parent(cx: &mut Context<Self>, parent: Handle<Self>) {
        let mut parent_element = parent.borrow_mut();
        parent_element.raw.append_child(&cx.raw).unwrap();
        parent_element.children.push(cx.handle());
    }

    pub fn on_click<O2>(
        cx: &mut Context<Self>,
        other: &Handle<O2>,
        slot: fn(&mut Context<O2>, MouseEvent),
    ) -> Binding
    where
        O2: 'static,
    {
        let handle = cx.handle();
        let callback: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
            handle.cx().emit(MouseEvent {});
        }));
        cx.raw
            .add_event_listener_with_callback("click", callback.as_ref().unchecked_ref())
            .unwrap();

        cx.handle().bind(other, slot)
    }
}

impl Object for Element {}

#[derive(Clone)]
pub struct MouseEvent {}

impl Signal<MouseEvent> for Element {}
