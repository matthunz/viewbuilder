use skia_safe::Color4f;
use viewbuilder::{Runtime, View};

#[tokio::main]
async fn main() {
    viewbuilder::transaction(|tx| {
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

    viewbuilder::run();
}
