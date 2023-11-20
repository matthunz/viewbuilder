use skia_safe::surfaces;
use taffy::geometry::Size;

use crate::{Element, Node};

pub struct LinearLayoutElement {
    pub(crate) nodes: Vec<Node>,
}

impl Element for LinearLayoutElement {
    fn layout(&mut self) -> taffy::prelude::Size<f64> {
        Size::default()
    }

    fn handle(&mut self, msg: crate::WindowMessage, output: &mut Vec<Box<dyn std::any::Any>>) {
        for node in &mut self.nodes {
            node.element.as_element_mut().handle(msg.clone(), output);
        }
    }

    fn render(&mut self, canvas: &mut skia_safe::Canvas) {
        let mut y = 0.;
        for node in &mut self.nodes {
            let size = node.element.as_element_mut().layout();

            let mut surface =
                surfaces::raster_n32_premul(skia_safe::ISize::new(2000, 200)).unwrap();
            node.element.as_element_mut().render(surface.canvas());
            let image = surface.image_snapshot();
            canvas.draw_image(image, skia_safe::Point::new(0., y), None);

            y += size.height as f32;
        }
    }
}
