use web_sys::Node;


pub trait Element {
    fn as_node(&self) -> Option<&Node>;
}
