use concoct::{Composer, Model};
use viewbuilder::{run, Window};

#[derive(Default)]
struct App {
    count: i32,
}

impl Model<()> for App {
    fn handle(&mut self, msg: ()) {
        self.count += 1;
    }
}

fn main() {
    let composer = Composer::new(App::default(), |model: &App| {
        Window::builder()
            .title(model.count.to_string())
            .on_event(|msg| {
                dbg!(msg);
            })
            .build()
    });
    run(composer)
}
