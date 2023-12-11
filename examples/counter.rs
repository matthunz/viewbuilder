use viewbuilder::{object, Object, Runtime};

#[derive(Default)]
pub struct Counter {
    value: i32,
}

#[object]
impl Counter {
    #[signal]
    fn value_changed(&mut self, value: i32);

    #[slot]
    pub fn set(&mut self, value: i32) {
        dbg!(value);
        self.value = value;
        self.value_changed(value);
    }
}

fn main() {
    let a = Counter::default().spawn();
    let b = Counter::default().spawn();

    a.value_changed().bind(&b, Counter::set);

    a.set(2);

    for _ in 0..2 {
        Runtime::current().run();
    }
}
