use crate::Transaction;
use skia_safe::{surfaces, Image};
use std::sync::mpsc;
use taffy::prelude::Size;
use tokio::{sync::oneshot, task};

#[derive(Clone)]
pub struct UserInterface {
    tx: mpsc::Sender<Box<dyn FnOnce(&mut Transaction) + Send>>,
}

impl UserInterface {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel::<Box<dyn FnOnce(&mut Transaction) + Send>>();

        task::spawn_blocking(move || {
            let mut transaction = Transaction::new();
            while let Ok(f) = rx.recv() {
                f(&mut transaction)
            }
        });

        Self { tx }
    }

    pub fn transaction(&self, f: impl FnOnce(&mut Transaction) + Send + 'static) {
        self.tx.send(Box::new(f)).unwrap();
    }

    pub async fn render(&self) -> Image {
        let (image_tx, image_rx) = oneshot::channel();

        self.transaction(|tx| {
            tx.taffy
                .compute_layout(tx.root, Size::max_content())
                .unwrap();

            let mut surface = surfaces::raster_n32_premul((300, 300)).unwrap();
            let canvas = surface.canvas();

            for (key, node) in &mut tx.nodes {
                node.layout = tx.taffy.layout(key).unwrap().clone();

                let image = node.element.as_element_mut().render(node.layout.size);
                canvas.draw_image(
                    image,
                    (
                        node.layout.location.x.floor(),
                        node.layout.location.y.floor(),
                    ),
                    None,
                );
            }

            let image = surface.image_snapshot();
            image_tx.send(image).unwrap();
        });

        image_rx.await.unwrap()
    }
}
