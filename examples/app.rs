use skia_safe::Color4f;
use viewbuilder::{Renderer, View};

#[tokio::main]
async fn main() {
    let renderer = Renderer::new();
    let ui = renderer.ui();

    ui.transaction(move |tx| {
        let child = tx.insert(
            View::builder()
                .background_color(Color4f::new(0., 1., 0., 1.))
                .build(),
        );

        tx.insert(
            View::builder()
                .background_color(Color4f::new(0., 1., 0., 1.))
                .child(child.key)
                .build(),
        );
    });

    renderer.run();
}
