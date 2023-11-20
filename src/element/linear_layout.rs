use kurbo::Point;
use skia_safe::surfaces;
use taffy::geometry::Size;

use crate::{Element, Node, WindowMessage};

pub struct LinearLayoutElement {
    pub(crate) nodes: Vec<Node>,
    pub(crate) points: Vec<(Point, Size<f64>)>,
}

impl Element for LinearLayoutElement {
    fn layout(&mut self) -> taffy::prelude::Size<f64> {
        let mut y = 0.;
        self.points.clear();
        for node in &mut self.nodes {
            let size = node.element.as_element_mut().layout();
            let point = Point::new(0., y);
            self.points.push((point, size));
            y += size.height;
        }
        Size {
            width: 1000.,
            height: y,
        }
    }

    fn handle(&mut self, msg: crate::WindowMessage, output: &mut Vec<Box<dyn std::any::Any>>) {
        match msg {
            WindowMessage::Click { position } => {
                for (node, (point, size)) in &mut self.nodes.iter_mut().zip(self.points.iter()) {
                    if position.x >= point.x
                        && position.x <= point.x + size.width
                        && position.y >= point.y
                        && position.y <= point.y + size.height
                    {
                        node.element.as_element_mut().handle(
                            WindowMessage::Click {
                                position: Point::new(point.x, position.y - point.y),
                            },
                            output,
                        );
                    }
                }
            }
        }
    }

    fn render(&mut self, canvas: &mut skia_safe::Canvas) {
        for (node, (point, size)) in &mut self.nodes.iter_mut().zip(self.points.iter()) {
            let mut surface = surfaces::raster_n32_premul(skia_safe::ISize::new(
                size.width.floor() as _,
                size.height.floor() as _,
            ))
            .unwrap();
            node.element.as_element_mut().render(surface.canvas());
            let image = surface.image_snapshot();
            canvas.draw_image(image, skia_safe::Point::new(0., point.y as _), None);
        }
    }
}
