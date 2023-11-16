use crate::Transaction;
use skia_safe::{surfaces, Image};
use std::sync::mpsc;
use tokio::{sync::oneshot, task};

pub struct UserInterface {
    tx: mpsc::Sender<Box<dyn FnOnce(&mut Transaction) + Send>>,
}

impl UserInterface {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel::<Box<dyn FnOnce(&mut Transaction) + Send>>();

        task::spawn_blocking(move || {
            let mut transaction = Transaction::default();
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
            let mut surface = surfaces::raster_n32_premul((300, 300)).unwrap();
            let canvas = surface.canvas();

            for elem in tx.elements.values_mut() {
                let image = elem.as_element_mut().render();
                canvas.draw_image(image, (0, 0), None);
            }

            let image = surface.image_snapshot();
            image_tx.send(image).unwrap();
        });

        image_rx.await.unwrap()
    }
}
