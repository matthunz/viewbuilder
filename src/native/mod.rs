use crate::{Model, Runtime, View};
use std::{borrow::Cow, mem};
use winit::{
    event_loop::{EventLoop, EventLoopBuilder, EventLoopWindowTarget},
    window::{Window as RawWindow, WindowBuilder},
};

pub fn run<M, T, VB, V, E>(model: T, view_builder: VB)
where
    T: Model<M>,
    VB: FnMut(&T) -> V,
    V: View<Native<E>, M>,
    E: 'static,
    M: Send + 'static,
{
    let event_loop: EventLoop<E> = EventLoopBuilder::with_user_event().build().unwrap();
    let state: Native<E> = Native { event_loop: None };
    let mut rt = Runtime::new(|_msg| {}, model, view_builder, state);

    event_loop
        .run(|_, target| {
            rt.state.event_loop = Some(unsafe { mem::transmute(target) });
            if rt.element.is_none() {
                rt.build();
            } else {
                rt.rebuild();
            }
        })
        .unwrap();
}

pub struct Native<T: 'static> {
    event_loop: Option<&'static EventLoopWindowTarget<T>>,
}

pub struct WindowData {
    title: Cow<'static, str>,
}

pub struct Window {
    data: Option<WindowData>,
}

impl Window {
    pub fn new() -> Self {
        Self {
            data: Some(WindowData {
                title: Cow::Borrowed("Viewbuilder"),
            }),
        }
    }
}

impl<E, M> View<Native<E>, M> for Window {
    type Element = (WindowData, RawWindow);

    fn build(&mut self, _cx: &mut crate::Context<M>, state: &mut Native<E>) -> Self::Element {
        let data = self.data.take().unwrap();
        let window = WindowBuilder::new()
            .with_title(data.title.clone())
            .build(state.event_loop.as_ref().unwrap())
            .unwrap();

        (data, window)
    }

    fn rebuild(
        &mut self,
        _cx: &mut crate::Context<M>,
        _state: &mut Native<E>,
        element: &mut Self::Element,
    ) {
        let data = self.data.take().unwrap();
        if data.title != element.0.title {
            element.0.title = data.title;
            element.1.set_title(&element.0.title)
        }
    }

    fn remove(
        &mut self,
        _cx: &mut crate::Context<M>,
        _state: &mut Native<E>,
        _element: Self::Element,
    ) {
        todo!()
    }
}
