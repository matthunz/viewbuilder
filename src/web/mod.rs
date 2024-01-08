use crate::{Context, ControlFlow, Model, Runtime, View};
use std::{cell::RefCell, rc::Rc, sync::Arc};
use web_sys::{wasm_bindgen::JsCast, Document, Element, Text};

pub mod html;

pub fn run<T, VB, V, M>(model: T, view_builder: VB)
where
    T: Model<M> + 'static,
    VB: FnMut(&T) -> V + 'static,
    V: View<Web, M>,
    V::Element: 'static,

    M: Send + 'static,
{
    let cell = Rc::new(RefCell::new(None::<Runtime<_, _, _, _, _>>));
    let cell_clone = cell.clone();
    let mut app = Runtime::new(
        move |msg| {
            let mut g = cell_clone.borrow_mut();
            let app = g.as_mut().unwrap();
            if let ControlFlow::Rebuild = app.handle(msg) {
                app.rebuild();
            }
        },
        model,
        view_builder,
        Web::default(),
    );
    app.build();

    *cell.borrow_mut() = Some(app);
}

pub struct Web {
    document: Document,
    parent: Element,
}

impl Default for Web {
    fn default() -> Self {
        let document = web_sys::window().unwrap().document().unwrap();
        Self {
            parent: document.body().unwrap().unchecked_into(),
            document,
        }
    }
}

pub struct HtmlAttributes {
    element: Element,
}

impl<M> View<Web, M> for &'static str {
    type Element = (Self, Text);

    fn build(&mut self, _cx: &mut Context<M>, tree: &mut Web) -> Self::Element {
        #[cfg(feature = "tracing")]
        crate::build_span!("&'static str");

        let text = tree.document.create_text_node(self);
        tree.parent.append_child(&text).unwrap();

        (self, text)
    }

    fn rebuild(&mut self, _cx: &mut Context<M>, _tree: &mut Web, element: &mut Self::Element) {
        #[cfg(feature = "tracing")]
        crate::rebuild_span!("&'static str");

        if *self != element.0 {
            element.0 = self;
            element.1.set_text_content(Some(self));
        }
    }

    fn remove(&mut self, _cx: &mut Context<M>, _state: &mut Web, element: Self::Element) {
        #[cfg(feature = "tracing")]
        crate::remove_span!("&'static str");

        element.1.remove();
    }
}

impl<M> View<Web, M> for String {
    type Element = (Self, Text);

    fn build(&mut self, _cx: &mut Context<M>, tree: &mut Web) -> Self::Element {
        #[cfg(feature = "tracing")]
        crate::build_span!("String");

        let text = tree.document.create_text_node(self);
        tree.parent.append_child(&text).unwrap();
        (self.clone(), text)
    }

    fn rebuild(&mut self, _cx: &mut Context<M>, _tree: &mut Web, element: &mut Self::Element) {
        #[cfg(feature = "tracing")]
        crate::rebuild_span!("String");

        if *self != element.0 {
            #[cfg(feature = "tracing")]
            tracing::event!(name: "Text change", tracing::Level::TRACE,  new = &*self, old = element.0);

            element.0 = self.clone();
            element.1.set_text_content(Some(self));
        }
    }

    fn remove(&mut self, _cx: &mut Context<M>, _state: &mut Web, element: Self::Element) {
        #[cfg(feature = "tracing")]
        crate::remove_span!("String");

        element.1.remove();
    }
}
