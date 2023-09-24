use slotmap::DefaultKey;

pub enum Event {
    Click(Click),
    MouseIn(MouseIn),
    MouseOut(MouseOut),
}

pub struct Click {
    pub target: DefaultKey,
}

pub struct MouseIn {
    pub target: DefaultKey,
}

pub struct MouseOut {
    pub target: DefaultKey,
}
