use crate::Element;
use skia_safe::{Image, Surface};
use slotmap::DefaultKey;
use taffy::style::Style;

#[derive(Default)]
pub struct View {
    children: Vec<DefaultKey>,
}

impl View {
    pub fn with_child(&mut self, key: DefaultKey) -> &mut Self {
        self.children.push(key);
        self
    }

    pub fn remove_child(&mut self, key: DefaultKey) {
        let idx = self
            .children
            .iter()
            .position(|child_key| key == *child_key)
            .unwrap();
        self.children.remove(idx);
    }
}

impl Element for View {
    fn children(&self) -> Option<Vec<DefaultKey>> {
        Some(self.children.clone())
    }

    fn layout(&mut self) -> Style {
        Style::default()
    }

    fn render(&mut self) -> Image {
        let mut surface = Surface::new_raster_n32_premul((300, 300)).unwrap();
        let mut canvas = surface.canvas();

        surface.image_snapshot()
    }
}
