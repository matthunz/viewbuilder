use std::time::{Duration, Instant};

use skia_safe::Color4f;
use taffy::{
    prelude::Size,
    style::{AlignItems, JustifyContent},
};
use viewbuilder::{render::UserEvent, Context, Element, Renderer};

#[tokio::main]
async fn main() {
    // TODO this is really early stage, an animation frame should be requested

    let mut tree = Context::default();
    let root = Element::new()
        .align_items(AlignItems::Center)
        .justify_content(JustifyContent::Center)
        .size(Size::from_points(100., 100.))
        .background_color(Color4f::new(0., 0., 1., 1.))
        .build(&mut tree);

    let renderer = Renderer::new();
    let tx = renderer.tx.clone();
    let notify = renderer.notify.clone();

    let mut is_forward = true;

    tokio::spawn(async move {
        let mut start = Instant::now();
        loop {
            let _min = 0.;
            let max = 500.;

            let elapsed = Instant::now() - start;
            let millis = elapsed.as_millis() as f32;
            let (begin, end) = if is_forward { (0., max) } else { (max, 0.) };
            let interpolated: f32 = interpolation::lerp(&begin, &end, &(millis / 500.));
            let size = interpolated.min(max).max(0.);

            if elapsed >= Duration::from_secs(1) {
                start = Instant::now();
                is_forward = !is_forward;
            }

            tx.send(UserEvent(Box::new(move |tree| {
                tree.node(root)
                    .set_size(Size::from_points(size as f32, size as f32))
            })))
            .unwrap();

            notify.notified().await;
        }
    });

    renderer.run(tree, root)
}
