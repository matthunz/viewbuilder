use accesskit::NodeBuilder;
use skia_safe::{Canvas, Paint, Rect};
use slotmap::DefaultKey;
use std::borrow::Cow;
use taffy::{prelude::Layout, style::Style, Taffy};

pub mod element;

pub use self::element::Element;

/// Kind of data type of a node.
#[derive(Debug, PartialEq, Eq)]
pub enum NodeKind {
    /// Element node kind.
    Element,

    /// Text node kind.
    Text,
}

/// Data type of a node.
pub enum NodeData {
    /// Element node.
    Element(Element),

    /// Text node.
    Text(Cow<'static, str>),
}

pub struct Node {
    /// Data type of the node.
    pub data: NodeData,

    /// Parent node id.
    pub parent: Option<DefaultKey>,

    /// Child node ids.
    pub children: Option<Vec<DefaultKey>>,

    /// Layout key for the taffy node.
    pub layout_key: Option<DefaultKey>,

    /// Absolute layout of the node, relative to the window.
    pub layout: Option<Layout>,
}

impl Node {
    pub fn new(data: NodeData) -> Self {
        Self {
            data,
            parent: None,
            children: None,
            layout_key: None,
            layout: None,
        }
    }

    pub fn text(content: impl Into<Cow<'static, str>>) -> Self {
        Self::new(NodeData::Text(content.into()))
    }

    pub fn kind(&self) -> NodeKind {
        match self.data {
            NodeData::Element { .. } => NodeKind::Element,
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

    pub fn paint(&mut self, canvas: &mut Canvas) {
        let layout = self.layout.as_ref().unwrap();
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
