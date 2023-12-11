use viewbuilder::{object, Object, Runtime};

#[derive(Default)]
pub struct Counter {
    value: i32,
}

#[object]
impl Counter {
    fn value_changed(&mut self, value: i32);

    #[slot]
    pub fn set_value(&mut self, value: i32) {
        self.value = value;
        self.value_changed(value);
    }
}

#[tokio::main]
async fn main() {
    let rt = Runtime::default();
    let _guard = rt.enter();

    let a = Counter::default().spawn();
    let b = Counter::default().spawn();

    a.value_changed().bind(&b, Counter::set_value);
    a.set_value(2);

    rt.run().await;

    assert_eq!(a.borrow().value, 2);
    assert_eq!(b.borrow().value, 2);
}
