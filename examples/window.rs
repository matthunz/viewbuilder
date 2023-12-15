use concoct::{Context, Object};
use viewbuilder::{event_loop::WindowEvent, EventLoop, Window};

struct App;

impl App {
    pub fn handle(cx: &mut Context<Self>, event: WindowEvent) {
        dbg!(event);
    }
}

impl Object for App {}

fn main() {
    let event_loop = EventLoop::<()>::new().start();

    let window = Window::new().start();
    Window::insert(&mut window.cx(), &event_loop);

    let app = App.start();
    window.bind(&app, App::handle);

    EventLoop::run(event_loop);
}
