use skia_safe::Color4f;
use viewbuilder::element::Text;

#[tokio::main]
async fn main() {
    viewbuilder::transaction(|ui| {
        ui.insert(
            Text::builder()
                .font_size(100.)
                .color(Color4f::new(1., 0., 0., 1.))
                .on_click(|text| {
                    viewbuilder::transaction(move |ui| ui[text].set_content(0, "Clicked!"))
                })
                .content("Hello World!")
                .build(),
        );
    });

    viewbuilder::run();
}
