use accesskit::NodeBuilder;
use skia_safe::{Canvas, Paint, Rect};
use slotmap::DefaultKey;
use std::borrow::Cow;
use taffy::{style::Style, Taffy};

pub mod element;
pub use element::Element;

#[derive(Debug, PartialEq, Eq)]
pub enum NodeKind {
    Container,
    Text,
}

pub enum NodeData {
    Element(Element),
    Text(Cow<'static, str>),
}

pub struct Node {
    pub data: NodeData,
    pub parent: Option<DefaultKey>,
    pub children: Option<Vec<DefaultKey>>,
    pub layout_key: Option<DefaultKey>,
}

impl Node {
    pub fn new(data: NodeData) -> Self {
        Self {
            data,
            parent: None,
            children: None,
            layout_key: None,
        }
    }

    pub fn text(content: impl Into<Cow<'static, str>>) -> Self {
        Self::new(NodeData::Text(content.into()))
    }

    pub fn kind(&self) -> NodeKind {
        match self.data {
            NodeData::Element { .. } => NodeKind::Container,
            NodeData::Text(_) => NodeKind::Text,
        }
    }

    pub fn semantics(&self) -> NodeBuilder {
        NodeBuilder::default()
    }

    pub fn layout(&mut self, taffy: &mut Taffy) {
        let mut style = Style::default();
        if let NodeData::Element(ref mut elem) = self.data {
            if let Some(size) = elem.size {
                style.size = size;
            }

            if let Some(flex_direction) = elem.flex_direction {
                style.flex_direction = flex_direction;
            }
        }

        if let Some(layout_key) = self.layout_key {
            taffy.set_style(layout_key, style).unwrap();
        } else {
            let layout_key = taffy.new_leaf(style).unwrap();
            self.layout_key = Some(layout_key);
        }
    }

    pub fn paint(&mut self, taffy: &Taffy, canvas: &mut Canvas) {
        let layout = taffy.layout(self.layout_key.unwrap()).unwrap();

        if let NodeData::Element(ref elem) = self.data {
            if let Some(background_color) = elem.background_color {
                let paint = Paint::new(background_color, None);
                canvas.draw_rect(
                    Rect::new(
                        layout.location.x,
                        layout.location.y,
                        layout.location.x + layout.size.width,
                        layout.location.y + layout.size.height,
                    ),
                    &paint,
                );
            }
        }
    }
}

impl From<&'static str> for Node {
    fn from(value: &'static str) -> Self {
        Self::text(value)
    }
}
