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

    let a = tree.insert(Box::new(TextElement::new("A!", &font)));
    let b = tree.insert(Box::new(TextElement::new("B!", &font)));
    let c = tree.insert(Box::new(TextElement::new("C!", &font)));
    let root = tree.insert(Box::new(ViewElement::new(vec![a, b, c])));

    Renderer.run(tree, root);
}
