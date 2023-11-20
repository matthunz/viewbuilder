use crate::{Node, View};

pub trait ViewGroup<'a, M> {
    fn build(&'a mut self, nodes: &mut Vec<Node>);

    fn rebuild(&'a mut self, nodes: &mut Vec<Node>);

    fn handle(&'a mut self, msg: M);
}

impl<'a, M, A, B> ViewGroup<'a, M> for (A, B)
where
    M: 'static,
    A: View<'a, M>,
    A::Element: 'static,
    B: View<'a, M>,
    B::Element: 'static,
{
    fn build(&'a mut self, nodes: &mut Vec<Node>) {
        nodes.push(Node::new(self.0.build()));
        nodes.push(Node::new(self.1.build()));
    }

    fn rebuild(&'a mut self, nodes: &mut Vec<Node>) {
        self.0
            .rebuild(nodes[0].element.as_any_mut().downcast_mut().unwrap());
        self.1
            .rebuild(nodes[1].element.as_any_mut().downcast_mut().unwrap());
    }

    fn handle(&'a mut self, _msg: M) {
        todo!()
    }
}
