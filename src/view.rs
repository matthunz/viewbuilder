use crate::Element;
use skia_safe::{Color4f, Image, Paint, Rect, Surface};
use slotmap::DefaultKey;
use taffy::{prelude::Size, style::Style};

#[derive(Default)]
pub struct View {
    children: Vec<DefaultKey>,
}

impl View {
    pub fn with_child(&mut self, key: DefaultKey) -> &mut Self {
        self.children.push(key);
        self
    }

    pub fn remove_child(&mut self, key: DefaultKey) {
        let idx = self
            .children
            .iter()
            .position(|child_key| key == *child_key)
            .unwrap();
        self.children.remove(idx);
    }
}

impl Element for View {
    fn children(&self) -> Option<Vec<DefaultKey>> {
        Some(self.children.clone())
    }

    fn layout(&mut self) -> Style {
        Style {
            size: Size::from_points(200., 200.),
            ..Default::default()
        }
    }

    fn render(&mut self, size: Size<f32>) -> Image {
        let mut surface = Surface::new_raster_n32_premul((
            size.width.floor() as i32 + 1,
            size.height.floor() as i32 + 1,
        ))
        .unwrap();
        let canvas = surface.canvas();

        let paint = Paint::new(Color4f::new(0., 1., 0., 1.), None);
        canvas.draw_rect(
            Rect {
                left: 0.,
                top: 0.,
                right: 200.,
                bottom: 200.,
            },
            &paint,
        );

        surface.image_snapshot()
    }
}
