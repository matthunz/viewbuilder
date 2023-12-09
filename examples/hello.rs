use viewbuilder::element::Text;

fn main() {
    viewbuilder::launch(Text::builder().font_size(100.).build("Hello Viewbuilder!"))
}
