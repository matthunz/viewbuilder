use crate::Element;
use skia_safe::{Color4f, Image, Paint, Rect, Surface};
use slotmap::DefaultKey;
use std::mem;
use taffy::{prelude::Size, style::Style};

#[derive(Default)]
pub struct Builder {
    view: View,
}

impl Builder {
    pub fn child(&mut self, key: DefaultKey) -> &mut Self {
        self.view.children.push(key);
        self
    }

    pub fn background_color(&mut self, color: impl Into<Option<Color4f>>) -> &mut Self {
        self.view.background_color = color.into();
        self
    }

    pub fn build(&mut self) -> View {
        mem::take(&mut self.view)
    }
}

#[derive(Default)]
pub struct View {
    children: Vec<DefaultKey>,
    background_color: Option<Color4f>,
}

impl View {
    pub fn builder() -> Builder {
        Builder::default()
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

        if let Some(background_color) = self.background_color {
            let paint = Paint::new(background_color, None);
            canvas.draw_rect(
                Rect {
                    left: 0.,
                    top: 0.,
                    right: size.width,
                    bottom: size.height,
                },
                &paint,
            );
        }

        surface.image_snapshot()
    }
}
