use viewbuilder::element::{LinearLayout, Text};
use viewbuilder::Window;

fn main() {
    let layout = viewbuilder::view(
        LinearLayout::builder()
            .child(Text::new("Hello"))
            .child(Text::new("World"))
            .build(),
    );

    Window::builder().title("Hello Example").build(layout);

    viewbuilder::run()
}
