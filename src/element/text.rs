use crate::Element;
use skia_safe::{Color4f, Font, FontStyle, Paint, TextBlob, Typeface};
use taffy::geometry::Size;

pub struct TextElement {
    text_blob: TextBlob,
}

impl TextElement {
    pub fn new(content: &str) -> Self {
        let typeface = Typeface::new("monospace", FontStyle::default()).unwrap();
        let font = Font::new(typeface, 100.);
        let text_blob = TextBlob::new(content, &font).unwrap();

        Self { text_blob }
    }
}

impl Element for TextElement {
    fn layout(&mut self) -> Size<f64> {
        Size {
            width: self.text_blob.bounds().width() as _,
            height: (self.text_blob.bounds().height() / 2.) as _,
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
