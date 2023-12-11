use kurbo::Point;
use viewbuilder::{App, Object, Window};
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
    let mut app = App::new();

    let window = Window {}.spawn();
    window
        .cursor_pos()
        .bind(&Example.spawn(), Example::set_cursor_pos);
    app.insert_window(window);

    app.run();
}
