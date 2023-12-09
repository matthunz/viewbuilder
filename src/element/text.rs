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
pub struct TextBuilder {
    font_size: Option<f64>,
    line_height: Option<f64>,
}

impl TextBuilder {
    pub fn font_size(&mut self, size: f64) -> &mut Self {
        self.font_size = Some(size);
        self
    }

    pub fn on_click(&mut self, _f: impl FnMut(ElementRef<Text>) + 'static) -> &mut Self {
        self
    }

    pub fn build(&mut self, content: impl Into<Cow<'static, str>>) -> Text {
        let mut text = Text::new(content);
        text.font_size = self.font_size;
        text.line_height = self.line_height;
        text
    }
}

pub struct Text {
    content: Cow<'static, str>,
    buffer: Option<Buffer>,
    font_size: Option<f64>,
    line_height: Option<f64>,
}

impl Text {
    pub fn new(content: impl Into<Cow<'static, str>>) -> Self {
        Self {
            content: content.into(),
            buffer: None,
            font_size: None,
            line_height: None,
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
        let cx = TextContext::current();
        let cache = &mut *cx.cache.borrow_mut();

        let font_size = self.font_size.unwrap_or(14.);
        let line_height = self.line_height.unwrap_or_else(|| font_size * 1.2);

        let metrics = Metrics::new(font_size as _, line_height as _);
        let mut buffer = Buffer::new(&mut cache.font_system, metrics);

        let mut buffer_ref = buffer.borrow_with(&mut cache.font_system);
        buffer_ref.set_size(1920.0, 1080.0);

        let attrs = Attrs::new().family(cosmic_text::Family::Monospace);
        buffer_ref.set_text(&self.content, attrs, Shaping::Advanced);

        buffer_ref.shape_until_scroll();

        let size = Size::new(100., line_height as f64);
        self.buffer = Some(buffer);
        size
    }

    fn render(&mut self, point: kurbo::Point, _size: Size, scene: &mut vello::SceneBuilder) {
        let cx = TextContext::current();
        let cache = &mut *cx.cache.borrow_mut();
        let text_color = Color::rgb(0, 255, 0);

        // Draw the buffer (for performance, instead use SwashCache directly)
        self.buffer.as_mut().unwrap().draw(
            &mut cache.font_system,
            &mut cache.swash_cache,
            text_color,
            |x, y, w, h, color| {
                scene.fill(
                    vello::peniko::Fill::EvenOdd,
                    Affine::translate((point.x, point.y)),
                    &Brush::Solid(vello::peniko::Color::rgba8(
                        color.r(),
                        color.g(),
                        color.b(),
                        color.a(),
                    )),
                    None,
                    &Rect::new(x as _, y as _, (x + w as i32) as _, (y + h as i32) as _),
                )
            },
        );
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
