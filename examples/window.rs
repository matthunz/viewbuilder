use concoct::{Context, Object};
use viewbuilder::{event_loop::WindowEvent, EventLoop, Window};

struct App;

impl App {
    pub fn event(_cx: &mut Context<Self>, event: WindowEvent) {
        dbg!(event);
    }
}

impl Object for App {}

fn main() {
    let event_loop = EventLoop::<()>::create();

    let window = Window::create();
    Window::insert(&mut window.cx(), &event_loop);

    let app = App.start();
    window.bind(&app, App::event);

    EventLoop::run(event_loop);
}
