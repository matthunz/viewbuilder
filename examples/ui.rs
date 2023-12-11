use kurbo::Point;
use viewbuilder::{ui::Window, Object, UserInterface};
use viewbuilder_macros::object;
use winit::event::{ElementState, MouseButton};

struct App;

#[object]
impl Object for App {
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
    let ui = UserInterface::new();
    let _guard = ui.enter();

    let window = Window {}.spawn_window();
    let app = App.spawn();

    window.cursor_moved().bind(&app, App::cursor_moved);
    window.mouse_event().bind(&app, App::mouse_event);

    ui.run();
}
