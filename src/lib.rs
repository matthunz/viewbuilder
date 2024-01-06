use concoct::{Composable, Composer, Context, Model};
use std::{any::Any, borrow::Cow, cell::RefCell, collections::HashMap, mem, rc::Rc};
use winit::{
    event::Event,
    event_loop::{EventLoop, EventLoopWindowTarget},
    window::{Window as RawWindow, WindowId},
};

#[derive(Clone)]
pub struct Runtime {
    window_handlers: Rc<RefCell<HashMap<WindowId, Box<dyn Fn(WindowEvent)>>>>,
    event_loop: &'static EventLoopWindowTarget<()>,
}

thread_local! {
    static RUNTIME: RefCell<Option<Runtime>> = RefCell::new(None);
}

impl Runtime {
    pub fn current() -> Self {
        RUNTIME
            .try_with(|rt| rt.borrow().as_ref().unwrap().clone())
            .unwrap()
    }
}

#[derive(Debug)]
pub enum WindowEvent {
    Mouse,
}

pub struct WindowBuilder<M> {
    title: Option<Cow<'static, str>>,
    handler: Option<Box<dyn Fn(WindowEvent) -> M>>,
}

impl<M> WindowBuilder<M> {
    pub fn title(&mut self, title: impl Into<Cow<'static, str>>) -> &mut Self {
        self.title = Some(title.into());
        self
    }

    pub fn on_event(&mut self, handler: impl Fn(WindowEvent) -> M + 'static) -> &mut Self {
        self.handler = Some(Box::new(handler));
        self
    }

    pub fn build(&mut self) -> Window<M> {
        Window {
            title: self.title.take(),
            handler: self.handler.take(),
        }
    }
}

pub struct Window<M> {
    title: Option<Cow<'static, str>>,
    handler: Option<Box<dyn Fn(WindowEvent) -> M>>,
}

impl<M> Window<M> {
    pub fn builder() -> WindowBuilder<M> {
        WindowBuilder {
            title: None,
            handler: None,
        }
    }
}

impl<M> Composable<M> for Window<M>
where
    M: Send + 'static,
{
    type State = RawWindow;

    fn compose(&mut self, cx: &mut Context<M>) -> Self::State {
        let rt = Runtime::current();

        let window = RawWindow::new(rt.event_loop).unwrap();

        if let Some(ref title) = self.title {
            window.set_title(title);
        }

        if let Some(handler) = self.handler.take() {
            let cx = cx.clone();
            rt.window_handlers
                .borrow_mut()
                .insert(window.id(), Box::new(move |event| cx.send(handler(event))));
        }

        window
    }

    fn recompose(&mut self, cx: &mut Context<M>, state: &mut Self::State) {
        if let Some(ref title) = self.title {
            if &state.title() != title {
                state.set_title(title);
            }
        }
    }
}

pub fn run<T, F, S, M, C>(mut composer: Composer<T, F, S, M>)
where
    T: Model<M> + 'static,
    F: FnMut(&T) -> C + 'static,
    C: Composable<M, State = S> + 'static,
    S: 'static,
    M: 'static,
{
    let event_loop = EventLoop::new();
    let mut is_composed = false;
    let window_handlers = Rc::new(RefCell::new(HashMap::new()));

    event_loop.run(move |event, event_loop, _| {
        let rt_window_handlers = window_handlers.clone();
        let rt = Runtime {
            event_loop: unsafe { mem::transmute(event_loop) },
            window_handlers: rt_window_handlers,
        };
        RUNTIME
            .try_with(|cell| *cell.borrow_mut() = Some(rt))
            .unwrap();

        composer.try_handle();

        if !is_composed {
            is_composed = true;
            composer.compose();
        } else {
            composer.recompose();
        }

        match event {
            Event::WindowEvent { window_id, event } => match event {
                winit::event::WindowEvent::MouseInput {
                    device_id,
                    state,
                    button,
                    modifiers,
                } => {
                    if let Some(handler) = window_handlers.borrow().get(&window_id) {
                        handler(WindowEvent::Mouse);
                    }
                }
                _ => {}
            },
            _ => {}
        }
    })
}
