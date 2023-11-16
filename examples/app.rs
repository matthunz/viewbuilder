use skia_safe::Color4f;
use viewbuilder::View;

#[tokio::main]
async fn main() {
    viewbuilder::transaction(|ui| {
        let child = ui.insert(
            View::builder()
                .background_color(Color4f::new(1., 0., 0., 1.))
                .build(),
        );

        ui.insert(
            View::builder()
                .background_color(Color4f::new(0., 1., 0., 1.))
                .on_click(move || {
                    viewbuilder::transaction(move |ui| ui[child].set_background_color(None))
                })
                .child(child.key)
                .build(),
        );
    });

    viewbuilder::run();
}
