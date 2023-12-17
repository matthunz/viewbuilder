use viewbuilder::{Context, View};

struct App;

impl View<()> for App {
    type Element = ();

    fn build(&mut self, cx: &mut Context<()>) -> Self::Element {
        cx.send(())
    }
}

fn app() -> impl View<i32> {
    App.map(|()| 2)
}

#[tokio::main]
async fn main() {
    let (mut cx, mut rx) = Context::new();
    app().build(&mut cx);

    dbg!(rx.recv().await);
}
