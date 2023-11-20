use crate::{AnyElement, Element};
use skia_safe::{surfaces, Image, Surface};
use taffy::geometry::Size;

pub struct Node {
    pub(crate) element: Box<dyn AnyElement>,
    pub(crate) surface: Option<Surface>,
}

impl Node {
    pub fn new(element: impl Element + 'static) -> Self {
        Self {
            element: Box::new(element),
            surface: None,
        }
    }

    pub fn paint(&mut self, size: Size<f64>) -> Image {
        let mut surface = surfaces::raster_n32_premul(skia_safe::ISize::new(
            size.width.floor() as _,
            size.height.floor() as _,
        ))
        .unwrap();

        self.element.as_element_mut().paint(surface.canvas());
        let image = surface.image_snapshot();
        self.surface = Some(surface);
        image
    }
}
