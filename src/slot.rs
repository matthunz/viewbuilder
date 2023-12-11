pub trait Slot<O, D> {
    fn handle(&mut self, object: &mut O, data: D);
}

macro_rules! impl_slot {
    ($($t:tt),*) => {
        impl<F: Fn(&mut O, $($t),*,), O, $($t),*> Slot<O, ($($t),*,)> for F {
            fn handle(&mut self, object: &mut O, data: ($($t),*,)) {
                #[allow(non_snake_case)]
                let ($($t),*,) = data;
                self(object, $($t),*)
            }
        }
    };
}

impl_slot!(T1);
impl_slot!(T1, T2);
impl_slot!(T1, T2, T3);
impl_slot!(T1, T2, T3, T4);
impl_slot!(T1, T2, T3, T4, T5);
impl_slot!(T1, T2, T3, T4, T5, T6);
impl_slot!(T1, T2, T3, T4, T5, T6, T7);
impl_slot!(T1, T2, T3, T4, T5, T6, T7, T8);
