use crate::{ui::Item, UserInterface};
use skia_safe::{surfaces, Image};

use taffy::prelude::{Layout, Size};
use tokio::{sync::mpsc, task};

#[derive(Clone)]
pub struct App {
    pub(crate) tx: mpsc::UnboundedSender<Box<dyn FnOnce(&mut UserInterface) + Send>>,
}

impl App {
    pub(crate) fn new(size: Size<i32>) -> (Self, mpsc::UnboundedReceiver<Image>) {
        let (tx, mut rx) = mpsc::unbounded_channel::<Box<dyn FnOnce(&mut UserInterface) + Send>>();
        let (image_tx, image_rx) = mpsc::unbounded_channel();

        task::spawn(async move {
            let mut ui = UserInterface::new();

            while let Some(f) = rx.recv().await {
                f(&mut ui);

                ui.taffy
                    .compute_layout(ui.root, Size::max_content())
                    .unwrap();

                let mut parents: Vec<Layout> = Vec::new();
                let mut levels = ui.levels_mut();
                while let Some(item) = levels.next() {
                    match item {
                        Item::Push(key) => {
                            let mut layout = levels.ui.taffy.layout(key).unwrap().clone();
                            if let Some(parent_layout) = parents.last() {
                                layout.location.x += parent_layout.location.x;
                                layout.location.x += parent_layout.location.x;
                            }
                            dbg!(layout);
                            levels.ui.nodes[key].layout = layout;
                            parents.push(layout);
                        }
                        Item::Pop => {
                            parents.pop();
                        }
                    }
                }

                let mut surface = surfaces::raster_n32_premul((size.width, size.height)).unwrap();
                let canvas = surface.canvas();

                for node in ui.nodes.values_mut() {
                    if let Some(image) = node.element.as_element_mut().render(node.layout.size) {
                        canvas.draw_image(
                            image,
                            (
                                node.layout.location.x.floor(),
                                node.layout.location.y.floor(),
                            ),
                            None,
                        );
                    }
                }

                let image = surface.image_snapshot();
                image_tx.send(image).unwrap();
            }
        });

        (Self { tx }, image_rx)
    }

    pub fn transaction(&self, f: impl FnOnce(&mut UserInterface) + Send + 'static) {
        self.tx.send(Box::new(f)).unwrap();
    }
}
