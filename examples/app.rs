use kurbo::Point;
use viewbuilder::{ui::Window, Object, UserInterface};
use viewbuilder_macros::object;
use winit::event::{ElementState, MouseButton};

struct Example;

#[object]
impl Object for Example {
    #[slot]
    fn cursor_moved(&mut self, point: Point) {
        dbg!(point);
    }

    #[slot]
    fn mouse_event(&mut self, state: ElementState, button: MouseButton) {
        dbg!(state, button);
    }
}

fn main() {
    let mut app = UserInterface::new();
    let _guard = app.enter();

    let window = Window {}.spawn();
    let example = Example.spawn();
    window.cursor_moved().bind(&example, Example::cursor_moved);
    window.mouse_event().bind(&example, Example::mouse_event);
    app.insert_window(window);

    app.run();
}
