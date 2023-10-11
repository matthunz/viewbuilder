use accesskit::NodeBuilder;

pub trait NodeFactory {
    fn semantics(&mut self) -> NodeBuilder;
}

pub fn from_fn<F>(node_builder: F) -> FromFn<F>
where
    F: FnMut() -> NodeBuilder,
{
    FromFn { f: node_builder }
}

pub struct FromFn<F> {
    f: F,
}

impl<F> NodeFactory for FromFn<F>
where
    F: FnMut() -> NodeBuilder,
{
    fn semantics(&mut self) -> NodeBuilder {
        (self.f)()
    }
}
