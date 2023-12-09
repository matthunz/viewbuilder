use vello::{
    kurbo::{Affine, Rect},
    peniko::Brush,
};

use crate::{Element, LocalElementRef};
use std::borrow::Cow;

pub enum TextMessage {
    SetContent(Cow<'static, str>),
}

pub struct Text {
    content: Cow<'static, str>,
}

impl Text {
    pub fn new(content: impl Into<Cow<'static, str>>) -> Self {
        Self {
            content: content.into(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn set_content(text: LocalElementRef<Self>, content: impl Into<Cow<'static, str>>) {
        text.send(TextMessage::SetContent(content.into()))
    }
}

impl Element for Text {
    type Message = TextMessage;

    fn lifecycle(&mut self, _cx: super::LifecycleContext, _lifecycle: super::Lifecycle) {}

    fn handle(&mut self, msg: Self::Message) {
        match msg {
            TextMessage::SetContent(content) => {
                self.content = content;
            }
        }
    }

    fn render(&mut self, mut scene: vello::SceneBuilder) {
        use cosmic_text::{Attrs, Buffer, Color, FontSystem, Metrics, Shaping, SwashCache};

        // A FontSystem provides access to detected system fonts, create one per application
        let mut font_system = FontSystem::new();

        // A SwashCache stores rasterized glyphs, create one per application
        let mut swash_cache = SwashCache::new();

        // Text metrics indicate the font size and line height of a buffer
        let metrics = Metrics::new(100.0, 100.0);

        // A Buffer provides shaping and layout for a UTF-8 string, create one per text widget
        let mut buffer = Buffer::new(&mut font_system, metrics);

        // Borrow buffer together with the font system for more convenient method calls
        let mut buffer = buffer.borrow_with(&mut font_system);

        // Set a size for the text buffer, in pixels
        buffer.set_size(1920.0, 1080.0);

        // Attributes indicate what font to choose
        let attrs = Attrs::new().family(cosmic_text::Family::Monospace);

        // Add some text!
        buffer.set_text(&self.content, attrs, Shaping::Advanced);

        // Perform shaping as desired
        buffer.shape_until_scroll();

        let text_color = Color::rgb(0, 255, 0);

        // Draw the buffer (for performance, instead use SwashCache directly)
        buffer.draw(&mut swash_cache, text_color, |x, y, w, h, color| {
            scene.fill(
                vello::peniko::Fill::EvenOdd,
                Affine::default(),
                &Brush::Solid(vello::peniko::Color::rgba8(
                    color.r(),
                    color.g(),
                    color.b(),
                    color.a(),
                )),
                None,
                &Rect::new(x as _, y as _, (x + w as i32) as _, (y + h as i32) as _),
            )
        });
    }
}
