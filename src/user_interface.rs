use crate::Transaction;
use skia_safe::{surfaces, Image};
use taffy::prelude::Size;
use tokio::{sync::mpsc, task};

#[derive(Clone)]
pub struct UserInterface {
    tx: mpsc::UnboundedSender<Box<dyn FnOnce(&mut Transaction) + Send>>,
}

impl UserInterface {
    pub(crate) fn new() -> (Self, mpsc::UnboundedReceiver<Image>) {
        let (tx, mut rx) = mpsc::unbounded_channel::<Box<dyn FnOnce(&mut Transaction) + Send>>();
        let (image_tx, image_rx) = mpsc::unbounded_channel();

        task::spawn(async move {
            let mut transaction = Transaction::new();

            while let Some(f) = rx.recv().await {
                f(&mut transaction);

                transaction
                    .taffy
                    .compute_layout(transaction.root, Size::max_content())
                    .unwrap();

                let mut surface = surfaces::raster_n32_premul((300, 300)).unwrap();
                let canvas = surface.canvas();

                for (key, node) in &mut transaction.nodes {
                    node.layout = transaction.taffy.layout(key).unwrap().clone();

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
            }
        });

        (Self { tx }, image_rx)
    }

    pub fn transaction(&self, f: impl FnOnce(&mut Transaction) + Send + 'static) {
        self.tx.send(Box::new(f)).unwrap();
    }
}
