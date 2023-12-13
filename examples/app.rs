use concoct::{ Handle, Object, Slot};
use viewbuilder::native::{
    view::{LinearLayout, Text},
    window, Window,
};
use winit::dpi::PhysicalSize;

struct App {
    width_text: Handle<Text>,
    height_text: Handle<Text>,
    size: PhysicalSize<u32>,
}

impl Object for App {}

impl Slot<window::Resized> for App {
    fn handle(&mut self, _cx: Handle<Self>, msg: window::Resized) {
        if msg.width != self.size.width {
            self.width_text.send(format!("Width: {}", msg.width).into());
            self.size.width = msg.width
        }

        if msg.height != self.size.height {
            self.height_text
                .send(format!("Height: {}", msg.height).into());
            self.size.height = msg.height
        }
    }
}

#[viewbuilder::main]
fn main() {
    let width_text = Text::default().start();
    let height_text = Text::default().start();

    let app = App {
        width_text: width_text.clone(),
        height_text: height_text.clone(),
        size: PhysicalSize::default(),
    }
    .start();

    let window = Window::new(LinearLayout::new((width_text, height_text))).start();
    window.bind(&app);
}
