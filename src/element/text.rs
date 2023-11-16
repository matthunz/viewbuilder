use crate::{Element, ElementRef};
use skia_safe::textlayout::{FontCollection, ParagraphBuilder, ParagraphStyle, TextStyle};
use skia_safe::{surfaces, Color4f, FontMgr, FontStyle, Image, Paint};
use slotmap::DefaultKey;
use std::borrow::Cow;
use std::mem;
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

    pub fn color(&mut self, color: Color4f) -> &mut Self {
        self.text.color = color;
        self
    }

    pub fn font_size(&mut self, font_size: f32) -> &mut Self {
        self.text.font_size = font_size;
        self
    }

    pub fn on_click(&mut self, _handler: impl FnMut(ElementRef<Text>) + 'static) -> &mut Self {
        self
    }

    pub fn build(&mut self) -> Text {
        mem::take(&mut self.text)
    }
}

pub struct Text {
    parts: Vec<Part>,
    font_size: f32,
    color: Color4f,
}

impl Default for Text {
    fn default() -> Self {
        Self {
            parts: Vec::new(),
            font_size: 24.,
            color: Color4f::new(0., 0., 0., 1.),
        }
    }
}

impl Text {
    pub fn builder() -> Builder {
        Builder::default()
    }

    pub fn set_content(&mut self, index: usize, text: impl Into<Cow<'static, str>>) {
        self.parts[index] = Part::Text(text.into());
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
        let mut surface = surfaces::raster_n32_premul((
            size.width.floor() as i32 + 1,
            size.height.floor() as i32 + 1,
        ))
        .unwrap();
        let canvas = surface.canvas();

        let mut text_style = TextStyle::new();
        let paint = Paint::new(self.color, None);
        text_style.set_font_size(self.font_size);
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
