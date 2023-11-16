use crate::{Element, ElementRef};
use skia_safe::{surfaces, Color4f, Image, Paint, Rect};
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

    pub fn on_click(&mut self, _handler: impl FnMut(ElementRef<Self>) + 'static) -> &mut Self {
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
    is_changed: bool,
    image: Option<Image>,
}

impl View {
    pub fn builder() -> Builder {
        Builder::default()
    }

    pub fn set_background_color(&mut self, color: impl Into<Option<Color4f>>) {
        self.background_color = color.into();
        self.is_changed = true;
    }

    pub fn add_child(&mut self, key: DefaultKey) {
        self.children.push(key);
        self.is_changed = true;
    }

    pub fn remove_child(&mut self, key: DefaultKey) {
        let idx = self
            .children
            .iter()
            .position(|child_key| key == *child_key)
            .unwrap();
        self.children.remove(idx);
        self.is_changed = true;
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

    fn render(&mut self, size: Size<f32>) -> Option<Image> {
        if let Some(image) = self.image.clone() {
            if !self.is_changed {
                return Some(image);
            }
        }

        let mut surface = surfaces::raster_n32_premul((
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

        let image = surface.image_snapshot();
        self.is_changed = false;
        self.image = Some(image.clone());
        Some(image)
    }
}
