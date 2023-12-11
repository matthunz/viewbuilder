use kurbo::Point;
use viewbuilder::{ui::Window, Object, UserInterface};
use viewbuilder_macros::object;

struct Example;

#[object]
impl Object for Example {
    #[slot]
    fn set_cursor_pos(&mut self, point: Point) {
        dbg!(point);
    }
}

fn main() {
    let mut app = UserInterface::new();
    let _guard = app.enter();

    let window = Window {}.spawn();
    let example = Example.spawn();
    window.cursor_pos().bind(&example, Example::set_cursor_pos);
    app.insert_window(window);

    app.run();
}
