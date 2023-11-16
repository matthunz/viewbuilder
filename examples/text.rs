use skia_safe::Color4f;
use viewbuilder::element::Text;

#[tokio::main]
async fn main() {
    viewbuilder::transaction(|ui| {
        ui.insert(
            Text::builder()
                .font_size(100.)
                .color(Color4f::new(1., 0., 0., 1.))
                .content("Hello World!")
                .build(),
        );
    });

    viewbuilder::run();
}
