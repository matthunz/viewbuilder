use skia_safe::{
    wrapper::NativeTransmutableWrapper, Color4f, Font, FontStyle, Paint, TextBlob, Typeface,
};
use crate::Element;

pub struct TextElement {}

impl Element for TextElement {
    fn render(&mut self, canvas: &mut skia_safe::Canvas) {
        let typeface = Typeface::new("monospace", FontStyle::default()).unwrap();
        let font = Font::new(typeface, 100.);
        let text_blob = TextBlob::new("Hello World!", &font).unwrap();
        let paint = Paint::new(Color4f::new(1., 0., 0., 1.), None);
        canvas.draw_text_blob(&text_blob, (50., 50.), &paint);
    }
}
