use crate::{Element, WindowMessage};
use kurbo::Point;
use skia_safe::{Color4f, Font, FontStyle, Paint, TextBlob, Typeface};
use taffy::geometry::Size;

pub struct TextElement<M> {
    text_blob: TextBlob,
    on_click: Option<Box<dyn FnMut(Point) -> M>>,
}

impl<M> TextElement<M> {
    pub fn new(content: &str, on_click: Option<Box<dyn FnMut(Point) -> M>>) -> Self {
        let typeface = Typeface::new("monospace", FontStyle::default()).unwrap();
        let font = Font::new(typeface, 100.);
        let text_blob = TextBlob::new(content, &font).unwrap();

        Self {
            text_blob,
            on_click,
        }
    }
}

impl<M: 'static> Element for TextElement<M> {
    fn layout(&mut self) -> Size<f64> {
        Size {
            width: self.text_blob.bounds().width() as _,
            height: (self.text_blob.bounds().height() / 2.) as _,
        }
    }

    fn handle(&mut self, msg: WindowMessage, output: &mut Vec<Box<dyn std::any::Any>>) {
        match msg {
            WindowMessage::Click { position } => {
                if let Some(ref mut f) = self.on_click {
                    output.push(Box::new(f(position)));
                }
            }
        }
    }

    fn render(&mut self, canvas: &mut skia_safe::Canvas) {
        let paint = Paint::new(Color4f::new(1., 0., 0., 1.), None);
        canvas.draw_text_blob(
            &self.text_blob,
            (0., self.text_blob.bounds().height() / 2.),
            &paint,
        );
    }
}
