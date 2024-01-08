use std::mem;

use web_sys::wasm_bindgen::{closure::Closure, JsCast};

use crate::{Context, HtmlAttributes, View, Web};

pub fn div<A, C, M>(attrs: A, content: C) -> Div<A, C>
where
    A: View<HtmlAttributes, M>,
    C: View<Web, M>,
{
    Div { attrs, content }
}

pub struct Div<A, C> {
    attrs: A,
    content: C,
}

impl<M, A, C> View<Web, M> for Div<A, C>
where
    A: View<HtmlAttributes, M>,
    C: View<Web, M>,
{
    type Element = (HtmlAttributes, A::Element);

    fn build(&mut self, cx: &mut Context<M>, tree: &mut Web) -> Self::Element {
        let element = tree.document.create_element("div").unwrap();
        tree.parent.append_child(&element).unwrap();

        let parent = mem::replace(&mut tree.parent, element);
        self.content.build(cx, tree);
        let element = mem::replace(&mut tree.parent, parent);

        let mut element_attrs = HtmlAttributes { element };
        let attrs = self.attrs.build(cx, &mut element_attrs);
        (element_attrs, attrs)
    }

    fn rebuild(&mut self, cx: &mut Context<M>, _tree: &mut Web, element: &mut Self::Element) {
        self.attrs.rebuild(cx, &mut element.0, &mut element.1)
    }
}

pub fn on_click<M, F>(handler: F) -> impl View<HtmlAttributes, M>
where
    F: FnMut() -> M + Clone + 'static,
    M: 'static,
{
    OnClick { f: handler }
}

pub struct OnClick<F> {
    f: F,
}

impl<M, F> View<HtmlAttributes, M> for OnClick<F>
where
    F: FnMut() -> M + Clone + 'static,
    M: 'static,
{
    type Element = Closure<dyn FnMut()>;

    fn build(&mut self, cx: &mut Context<M>, tree: &mut HtmlAttributes) -> Self::Element {
        let mut handler = self.f.clone();
        let cx = cx.clone();
        let closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
            let msg = handler();
            cx.send(msg);
        }));
        tree.element
            .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
            .unwrap();
        closure
    }

    fn rebuild(
        &mut self,
        _cx: &mut Context<M>,
        _tree: &mut HtmlAttributes,
        _element: &mut Self::Element,
    ) {
    }
}
