use std::cell::RefCell;
use std::rc::Rc;

use viewbuilder::element::{LinearLayout, Text};
use viewbuilder::Window;

fn main() {
    let label = viewbuilder::view(Text::new("0"));
    let count = Rc::new(RefCell::new(0));
    let count_clone = count.clone();

    let layout = viewbuilder::view(
        LinearLayout::builder()
            .child(label)
            .child(
                Text::builder()
                    .on_click(move || {
                        let mut n = count.borrow_mut();
                        *n += 1;
                        label
                            .get()
                            .borrow_mut()
                            .set_content(format!("High fives: {}", n));
                    })
                    .build("More!"),
            )
            .child(
                Text::builder()
                    .on_click(move || {
                        let mut n = count_clone.borrow_mut();
                        *n -= 1;
                        label
                            .get()
                            .borrow_mut()
                            .set_content(format!("High fives: {}", n));
                    })
                    .build("Less!"),
            )
            .build(),
    );

    Window::builder().title("Counter Example").build(layout);

    viewbuilder::run()
}
