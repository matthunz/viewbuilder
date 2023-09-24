use crate::NodeKey;

pub enum Event {
    Click(Click),
    MouseIn(MouseIn),
    MouseOut(MouseOut),
}

pub struct Click {
    pub target: NodeKey,
}

pub struct MouseIn {
    pub target: NodeKey,
}

pub struct MouseOut {
    pub target: NodeKey,
}
