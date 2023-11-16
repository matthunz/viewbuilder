use std::borrow::Cow;
use std::mem;
use crate::Element;
use skia_safe::textlayout::{FontCollection, ParagraphBuilder, ParagraphStyle, TextStyle};
use skia_safe::{surfaces, Color4f, FontMgr, FontStyle, Image, Paint};
use taffy::prelude::Size;
use taffy::style::Style;

enum Part {
    Style(TextStyle),
    Text(Cow<'static, str>),
}

#[derive(Default)]
pub struct Builder {
    text: Text,
}

impl Builder {
    pub fn content(&mut self, text: impl Into<Cow<'static, str>>) -> &mut Self {
        self.text.parts.push(Part::Text(text.into()));
        self
    }

    pub fn build(&mut self) -> Text {
        mem::take(&mut self.text)
    }
}

#[derive(Default)]
pub struct Text {
    parts: Vec<Part>,
}

impl Text {
    pub fn builder() -> Builder {
        Builder::default()
    }
}

impl Element for Text {
    fn children(&self) -> Option<Vec<slotmap::DefaultKey>> {
        None
    }

    fn layout(&mut self) -> taffy::style::Style {
        Style {
            size: Size::from_points(1000., 200.),
            ..Default::default()
        }
    }

    fn render(&mut self, size: taffy::prelude::Size<f32>) -> Image {
        dbg!(size);
        let mut surface = surfaces::raster_n32_premul((
            size.width.floor() as i32 + 1,
            size.height.floor() as i32 + 1,
        ))
        .unwrap();
        let canvas = surface.canvas();

        let mut text_style = TextStyle::new();
        let paint = Paint::new(Color4f::new(0., 0., 0., 1.), None);
        text_style.set_font_size(100.);
        text_style.set_font_style(FontStyle::default());
        text_style.set_font_families(&["monospace"]);
        text_style.set_foreground_paint(&paint);

        let mut style = ParagraphStyle::new();
        style.set_text_style(&text_style);

        let font_manager = FontMgr::new();
        let mut font_collection = FontCollection::new();
        font_collection.set_default_font_manager(font_manager, "monospace");

        let mut builder = ParagraphBuilder::new(&style, font_collection);

        for part in &self.parts {
            match part {
                Part::Style(style) => {
                    builder.push_style(style);
                }
                Part::Text(text) => {
                    builder.add_text(text);
                    builder.pop();
                }
            }
        }

        let mut paragraph = builder.build();
        paragraph.layout(size.width);
        paragraph.paint(canvas, (0, 0));

        surface.image_snapshot()
    }
}
