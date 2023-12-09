use crate::{Element, ElementRef};
use cosmic_text::{Attrs, Buffer, Color, FontSystem, Metrics, Shaping, SwashCache};
use kurbo::Size;
use slotmap::DefaultKey;
use std::{borrow::Cow, cell::RefCell, rc::Rc};
use vello::{
    kurbo::{Affine, Rect},
    peniko::Brush,
};

#[derive(Default)]
pub struct TextBuilder {}

impl TextBuilder {
    pub fn on_click(&mut self, f: impl FnMut(ElementRef<Text>) + 'static) -> &mut Self {
        self
    }

    pub fn build(&mut self, content: impl Into<Cow<'static, str>>) -> Text {
        Text::new(content)
    }
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

    pub fn builder() -> TextBuilder {
        TextBuilder::default()
    }

    pub fn set_content(&mut self, _content: impl Into<Cow<'static, str>>) {}
}

impl Element for Text {
    fn children(&self) -> Option<Box<[DefaultKey]>> {
        None
    }

    fn layout(&mut self, _min: Option<Size>, _max: Option<Size>) -> Size {
        Size::new(100., 100.)
    }

    fn render(&mut self, point: kurbo::Point, size: Size, scene: &mut vello::SceneBuilder) {
        let cx = TextContext::current();
        let cache = &mut *cx.cache.borrow_mut();

        // Text metrics indicate the font size and line height of a buffer
        let metrics = Metrics::new(100.0, 100.0);

        // A Buffer provides shaping and layout for a UTF-8 string, create one per text widget
        let mut buffer = Buffer::new(&mut cache.font_system, metrics);

        // Borrow buffer together with the font system for more convenient method calls
        let mut buffer = buffer.borrow_with(&mut cache.font_system);

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
        buffer.draw(&mut cache.swash_cache, text_color, |x, y, w, h, color| {
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

pub struct TextCache {
    font_system: FontSystem,
    swash_cache: SwashCache,
}

impl Default for TextCache {
    fn default() -> Self {
        Self {
            font_system: FontSystem::new(),
            swash_cache: SwashCache::new(),
        }
    }
}

#[derive(Clone)]
pub struct TextContext {
    pub cache: Rc<RefCell<TextCache>>,
}

impl TextContext {
    pub fn current() -> Self {
        thread_local! {
            static CONTEXT: TextContext = TextContext { cache: Default::default() };
        }
        CONTEXT.try_with(|cx| cx.clone()).unwrap()
    }
}
