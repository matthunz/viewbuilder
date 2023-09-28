use crate::{Context, Window};
use slotmap::{DefaultKey, SlotMap};
use std::{
    collections::HashMap,
    future::Future,
    sync::{mpsc, Arc},
    time::{Duration, Instant},
};
use tokio::sync::Notify;
use winit::{
    event::Event,
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    window::WindowId,
};

/// Error returned from a failed update.
#[derive(Clone, Copy, Debug)]
pub struct UpdateError;

type Update<T> = Box<dyn FnOnce(&mut Context<T>) + Send>;

/// Update context for a renderer.
pub struct Updater<T> {
    tx: mpsc::Sender<UserEvent<T>>,
}

impl<T> Updater<T> {
    /// Send an update to the UI tree.
    pub fn update(&self, f: Update<T>) -> Result<(), UpdateError> {
        self.tx.send(UserEvent::Update(f)).map_err(|_| UpdateError)
    }
}

/// Scope context for a renderer.
///
/// This provides a way to request animation frames and send updates.
pub struct Scope<T> {
    updater: Updater<T>,
    notify: Arc<Notify>,
}

impl<T> Scope<T> {
    /// Request an animation frame from the renderer.
    pub async fn request_frame(&self) -> Result<(), ()> {
        self.updater
            .tx
            .send(UserEvent::FrameRequest)
            .map_err(|_| ())?;
        self.notify.notified().await;
        Ok(())
    }

    /// Send an update to the UI tree.
    pub fn update(&self, f: Update<T>) -> Result<(), UpdateError> {
        self.updater.update(f)
    }
}

pub(crate) enum UserEvent<T> {
    Update(Update<T>),
    FrameRequest,
}

struct RendererWindow {
    window: Window,
    context_key: DefaultKey,
}

/// Application renderer.
pub struct Renderer<T: 'static = ()> {
    windows: HashMap<WindowId, RendererWindow>,
    contexts: SlotMap<DefaultKey, Context<T>>,
    pub(crate) event_loop: EventLoop<UserEvent<T>>,
    tx: mpsc::Sender<UserEvent<T>>,
    rx: mpsc::Receiver<UserEvent<T>>,
    notify: Arc<Notify>,
}

impl<T> Default for Renderer<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Renderer<T> {
    /// Create a new renderer.
    pub fn new() -> Self {
        let event_loop = EventLoopBuilder::with_user_event().build();
        let (tx, rx) = mpsc::channel();

        Self {
            windows: HashMap::new(),
            contexts: SlotMap::new(),
            event_loop,
            tx,
            rx,
            notify: Arc::new(Notify::default()),
        }
    }

    /// Insert a window into the renderer with its context key, returning its ID.
    ///
    /// This will bind a window to context for re-rendering.
    pub fn insert_window(&mut self, window: Window, context_key: DefaultKey) -> WindowId {
        let id = window.window.id();
        self.windows.insert(
            id,
            RendererWindow {
                window,
                context_key,
            },
        );
        id
    }

    /// Insert a context into the renderer, returning its key.
    pub fn insert_context(&mut self, cx: Context<T>) -> DefaultKey {
        self.contexts.insert(cx)
    }

    /// Create an updater handle to the renderer.
    pub fn updater(&self) -> Updater<T> {
        Updater {
            tx: self.tx.clone(),
        }
    }

    /// Create a scope handle to the renderer.
    pub fn scope(&self) -> Scope<T> {
        Scope {
            updater: self.updater(),
            notify: self.notify.clone(),
        }
    }

    /// Request a transition animation.
    pub fn animation(
        &self,
        min: f32,
        max: f32,
        f: impl Fn(&mut Context<T>, f32) + Send + Sync + 'static,
    ) -> impl Future<Output = ()> {
        let scope = self.scope();

        let mut is_forward = true;
        let mut start = Instant::now();

        async move {
            let f = Arc::new(f);
            loop {
                let elapsed = Instant::now() - start;
                let millis = elapsed.as_millis() as f32;

                let (begin, end) = if is_forward { (min, max) } else { (max, min) };
                let interpolated: f32 = interpolation::lerp(&begin, &end, &(millis / max));
                let size = interpolated.min(max).max(min);

                if elapsed >= Duration::from_secs(1) {
                    start = Instant::now();
                    is_forward = !is_forward;
                }

                let f2 = f.clone();
                scope.update(Box::new(move |cx| f2(cx, size))).unwrap();
                scope.request_frame().await.unwrap();
            }
        }
    }

    /// Run the renderer.
    pub fn run(mut self) -> ! {
        let mut previous_frame_start = Instant::now();
        let proxy = self.event_loop.create_proxy();

        self.event_loop.run(move |event, _, control_flow| {
            let frame_start = Instant::now();
            let mut draw_frame = false;

            if let Ok(event) = self.rx.try_recv() {
                proxy.send_event(event).ok().unwrap();
            }

            match event {
                Event::LoopDestroyed => {}
                Event::WindowEvent { window_id, event } => {
                    let window_cx = self.windows.get_mut(&window_id).unwrap();
                    let cx = &mut self.contexts[window_cx.context_key];

                    match window_cx.window.handle(cx, window_cx.window.root, event) {
                        Ok(cell) => {
                            if let Some(cf) = cell {
                                *control_flow = cf;
                            }
                        }
                        Err(_error) => {
                            *control_flow = ControlFlow::Exit;
                        }
                    }
                }
                Event::RedrawRequested(_) => {
                    draw_frame = true;
                }
                Event::UserEvent(UserEvent::Update(_update)) => {
                    // update(cx)
                }
                Event::UserEvent(UserEvent::FrameRequest) => {
                    draw_frame = true;
                }
                _ => (),
            }

            let frame_duration = Duration::from_millis(16);
            if frame_start - previous_frame_start > frame_duration {
                draw_frame = true;
                previous_frame_start = frame_start;
            }
            if draw_frame {
                self.notify.notify_waiters();
                for window_cx in self.windows.values_mut() {
                    let cx = &mut self.contexts[window_cx.context_key];
                    window_cx.window.paint(cx);
                }
            }

            *control_flow = ControlFlow::WaitUntil(previous_frame_start + frame_duration)
        })
    }
}
