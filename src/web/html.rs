use crate::{Context, HtmlAttributes, View, Web};
use std::{fmt, marker::PhantomData, mem};
use web_sys::wasm_bindgen::{closure::Closure, JsCast};

macro_rules! tags {
    ($($name:tt),*) => {
        $(
            pub fn $name<A, C, M>(attrs: A, content: C) -> Element<&'static str, A, C, M>
            where
                A: View<HtmlAttributes, M>,
                C: View<Web, M>,
            {
                element(stringify!($name), attrs, content)
            }
        )*
    };
}

tags!(
    a, abbr, address, area, article, aside, audio, b, base, bdi, bdo, blockquote, body, br, button,
    canvas, caption, cite, code, col, colgroup, data, datalist, dd, del, details, dfn, dialog, div,
    dl, dt, em, embed, fieldset, figcaption, figure, footer, form, h1, h2, h3, h4, h5, h6, head,
    header, hr, html, i, iframe, img, input, ins, kbd, label, legend, li, link, main, map, mark,
    meta, meter, nav, noscript, object, ol, optgroup, option, output, p, param, picture, pre,
    progress, q, rp, rt, ruby, s, samp, script, section, select, small, source, span, strong, sub,
    summary, sup, svg, table, tbody, td, template, textarea, tfoot, th, thead, time, title, tr,
    track, u, ul, var, video, wbr
);

pub fn element<T, A, C, M>(tag: T, attrs: A, content: C) -> Element<T, A, C, M>
where
    T: AsRef<str>,
    A: View<HtmlAttributes, M>,
    C: View<Web, M>,
{
    Element {
        tag,
        attrs,
        content,
        _marker: PhantomData,
    }
}

pub struct Element<T, A, C, M> {
    tag: T,
    attrs: A,
    content: C,
    _marker: PhantomData<M>,
}

impl<M, T, A, C> View<Web, M> for Element<T, A, C, M>
where
    T: AsRef<str>,
    A: View<HtmlAttributes, M>,
    C: View<Web, M>,
{
    type Element = (HtmlAttributes, A::Element);

    fn build(&mut self, cx: &mut Context<M>, tree: &mut Web) -> Self::Element {
        let element = tree.document.create_element(self.tag.as_ref()).unwrap();
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

pub fn on_click<M, F>(f: F) -> impl View<HtmlAttributes, M>
where
    F: FnMut() -> M + Clone + 'static,
    M: 'static,
{
    handler("click", f)
}

pub fn on_double_click<M, F>(f: F) -> impl View<HtmlAttributes, M>
where
    F: FnMut() -> M + Clone + 'static,
    M: 'static,
{
    handler("dblclick", f)
}

pub fn handler<M, T, F>(event: T, handler: F) -> impl View<HtmlAttributes, M>
where
    T: AsRef<str>,
    F: FnMut() -> M + Clone + 'static,
    M: 'static,
{
    Handler { event, f: handler }
}

pub struct Handler<T, F> {
    event: T,
    f: F,
}

impl<M, T, F> View<HtmlAttributes, M> for Handler<T, F>
where
    T: AsRef<str>,
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

pub fn style<M, V>(view: V) -> Style<V, M>
where
    V: View<StyleTree, M>,
{
    Style {
        view,
        _marker: PhantomData,
    }
}

pub struct StyleTree {
    s: String,
}

pub struct Style<V, M> {
    view: V,
    _marker: PhantomData<M>,
}

impl<M, V> View<HtmlAttributes, M> for Style<V, M>
where
    V: View<StyleTree, M>,
{
    type Element = StyleTree;

    fn build(&mut self, cx: &mut Context<M>, tree: &mut HtmlAttributes) -> Self::Element {
        let mut element = StyleTree { s: String::new() };
        self.view.build(cx, &mut element);

        tree.element.set_attribute("style", &element.s).unwrap();

        element
    }

    fn rebuild(
        &mut self,
        cx: &mut Context<M>,
        tree: &mut HtmlAttributes,
        element: &mut Self::Element,
    ) {
        todo!()
    }
}

pub fn css<K, V>(key: K, value: V) -> Css<K, V>
where
    K: fmt::Display + Clone,
    V: fmt::Display + Clone,
{
    Css { key, value }
}

pub struct Css<K, V> {
    key: K,
    value: V,
}

impl<M, K, V> View<StyleTree, M> for Css<K, V>
where
    K: fmt::Display + Clone,
    V: fmt::Display + Clone,
{
    type Element = (K, V);

    fn build(&mut self, cx: &mut Context<M>, tree: &mut StyleTree) -> Self::Element {
        tree.s.push_str(&format!("{}: {};", &self.key, &self.value));
        (self.key.clone(), self.value.clone())
    }

    fn rebuild(&mut self, cx: &mut Context<M>, tree: &mut StyleTree, element: &mut Self::Element) {
        todo!()
    }
}
