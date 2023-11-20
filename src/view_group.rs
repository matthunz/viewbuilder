use crate::{Node, View};

pub trait ViewGroup<'a, M> {
    fn build(&'a mut self, nodes: &mut Vec<Node>);

    fn rebuild(&'a mut self, nodes: &mut Vec<Node>);
}

macro_rules! impl_view_group_for_tuple {
    ($($generic:ident: $idx:tt),*) => {
        impl<'a, Msg, $($generic),*> ViewGroup<'a, Msg> for ($($generic),*)
        where
            Msg: 'static,
            $($generic: View<'a, Msg> + 'a),*
        {
            fn build(&'a mut self, nodes: &mut Vec<Node>) {
                $(nodes.push(Node::new(self.$idx.build()));)*
            }

            #[allow(unused_assignments)]
            fn rebuild(&'a mut self, nodes: &mut Vec<Node>) {
                let mut index = 0;
                $(self.$idx.rebuild(nodes[index].element.as_any_mut().downcast_mut().unwrap()); index += 1;)*
            }
        }
    };
}

impl_view_group_for_tuple!(A: 0, B: 1);
impl_view_group_for_tuple!(A: 0, B: 1, C: 2);
impl_view_group_for_tuple!(A: 0, B: 1, C: 2, D: 3);
impl_view_group_for_tuple!(A: 0, B: 1, C: 2, D: 3, E: 4);
impl_view_group_for_tuple!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5);
impl_view_group_for_tuple!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6);
impl_view_group_for_tuple!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7);
impl_view_group_for_tuple!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8);
impl_view_group_for_tuple!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9);
impl_view_group_for_tuple!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9, K: 10);
impl_view_group_for_tuple!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9, K: 10, L: 11);
impl_view_group_for_tuple!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9, K: 10, L: 11, M: 12);
impl_view_group_for_tuple!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9, K: 10, L: 11, M: 12, N: 13);
impl_view_group_for_tuple!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9, K: 10, L: 11, M: 12, N: 13, O: 14);
impl_view_group_for_tuple!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9, K: 10, L: 11, M: 12, N: 13, O: 14, P: 15);
