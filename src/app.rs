use crate::UserInterface;
use skia_safe::{surfaces, Image};
use taffy::prelude::Size;
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

                ui.layout();

                let mut surface = surfaces::raster_n32_premul((size.width, size.height)).unwrap();
                let canvas = surface.canvas();
                ui.render(canvas);

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
