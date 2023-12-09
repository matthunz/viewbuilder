use viewbuilder::element::{LinearLayout, Text};
use viewbuilder::Window;

fn main() {
    let layout = viewbuilder::view(
        LinearLayout::builder()
            .child(Text::new("Hello World!"))
            .build(),
    );
    Window::new(layout);
}
