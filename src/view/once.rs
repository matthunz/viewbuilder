use crate::{Context, View};

pub fn once<C, M>(composable: C) -> Once<C>
where
    C: View<M>,
{
    Once { composable }
}

pub struct Once<C> {
    composable: C,
}

impl<M, C> View<M> for Once<C>
where
    C: View<M>,
{
    type Element = C::Element;

    fn build(&mut self, cx: &mut Context<M>) -> Self::Element {
        self.composable.build(cx)
    }

    fn rebuild(&mut self, _cx: &mut Context<M>, _state: &mut Self::Element) {}
}
