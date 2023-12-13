use concoct::{Context, Handle, Object, Slot};
use viewbuilder::{
    view::{LinearLayout, Text},
    window, UserInterface, Window,
};
use winit::dpi::PhysicalSize;

struct App {
    width_text: Handle<Text>,
    height_text: Handle<Text>,
    size: PhysicalSize<u32>,
}

impl Object for App {}

impl Slot<window::Resized> for App {
    fn handle(&mut self, _cx: Context<Self>, msg: window::Resized) {
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
    let width_text = Text::default().spawn();
    let height_text = Text::default().spawn();

    let app = App {
        width_text: width_text.clone(),
        height_text: height_text.clone(),
        size: PhysicalSize::default(),
    }
    .spawn();

    let window = Window::new(LinearLayout::new((width_text, height_text))).spawn();
    window.bind(&app);
}
