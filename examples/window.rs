use concoct::Object;
use viewbuilder::window::EventLoop;

fn main() {
    let event_loop = EventLoop::<()>::new().start();
    EventLoop::run(event_loop);
}
