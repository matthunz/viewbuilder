use concoct::Object;
use viewbuilder::window::{EventLoop, Window};

fn main() {
    let event_loop = EventLoop::<()>::new().start();

    let a = Window::new().start();
    Window::insert(&mut a.cx(), &event_loop);

    a.listen(|msg| {
        dbg!(msg);
    });

    EventLoop::run(event_loop);
}
