use std::{
    thread::{self, sleep},
    time::{Duration, Instant},
};

use skia_safe::Color4f;
use taffy::{
    prelude::Size,
    style::{AlignItems, JustifyContent},
};
use viewbuilder::{render::UserEvent, Element, Renderer, Tree};

fn main() {
    // TODO this is really early stage, an animation frame should be requested

    let mut tree = Tree::default();
    let root = Element::new()
        .align_items(AlignItems::Center)
        .justify_content(JustifyContent::Center)
        .size(Size::from_points(100., 100.))
        .background_color(Color4f::new(0., 0., 1., 1.))
        .build(&mut tree);

    let renderer = Renderer::new();
    let tx = renderer.tx.clone();

    thread::spawn(move || {
        let start = Instant::now();
        loop {
            let t = (Instant::now() - start).as_millis() as f32;
            let s: f32 = interpolation::lerp(&0., &100., &(t / 1000.));

            tx.send(UserEvent(Box::new(move |tree| {
                tree.element(root)
                    .set_size(Size::from_points(s as f32, s as f32))
            })))
            .unwrap();

            sleep(Duration::from_millis(5))
        }
    });

    renderer.run(tree, root)
}
