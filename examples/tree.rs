use skia_safe::{Font, Typeface};
use viewbuilder::{element::TextElement, Tree};

fn main() {
    let mut tree = Tree::default();

    let typeface = Typeface::new("Arial", Default::default()).unwrap();
    let font = Font::new(typeface, 100.);
    let root = tree.insert(Box::new(TextElement::new("Hello World!", &font)));

    tree.layout(root)
}
