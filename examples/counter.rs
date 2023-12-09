use viewbuilder::element::{LinearLayout, Text};
use viewbuilder::Window;

fn main() {
    let mut count = 0;
    let layout = viewbuilder::view(
        LinearLayout::builder()
            .child(Text::builder().font_size(100.).build("Counter"))
            .child(
                Text::builder()
                    .font_size(50.)
                    .on_click(move |text| {
                        count += 1;
                        text.get()
                            .borrow_mut()
                            .set_content(format!("High fives: {}", count));
                    })
                    .build("High fives: 0"),
            )
            .build(),
    );

    Window::builder().title("Counter Example").build(layout);

    viewbuilder::run()
}
