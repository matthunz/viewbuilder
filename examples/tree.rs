use skia_safe::{Font, Typeface};
use viewbuilder::{
    element::{TextElement, ViewElement},
    render::Renderer,
    tree::Tree,
};

fn main() {
    let mut tree = Tree::default();

    let typeface = Typeface::new("Arial", Default::default()).unwrap();
    let font = Font::new(typeface, 100.);

    let a = tree.insert(Box::new(TextElement::new("Less!", &font)));
    let b = tree.insert(Box::new(TextElement::new("More!", &font)));
    let root = tree.insert(Box::new(ViewElement::new(vec![a, b])));

    Renderer.run(tree, root);
}
