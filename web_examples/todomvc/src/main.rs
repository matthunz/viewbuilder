use viewbuilder::{
    web::{self, html, Web},
    ControlFlow, Model, View,
};

enum Message {
    Check { id: u64, is_checked: bool },
    Editing { id: u64, is_editing: bool },
    Remove { id: u64 },
}

pub struct Entry {
    id: u64,
    content: String,
    is_done: bool,
    is_editing: bool,
}

fn view(model: &App) -> impl View<Web, Message> {
    model
        .entries
        .iter()
        .map(|entry| (entry.id, view_entry(entry)))
        .collect::<Vec<_>>()
}

fn view_entry(entry: &Entry) -> impl View<Web, Message> {
    let id = entry.id;
    let is_done = entry.is_done;

    html::li(
        (),
        (
            html::input(
                html::on_click(move || Message::Check {
                    id,
                    is_checked: !is_done,
                }),
                (),
            ),
            html::label(
                html::on_double_click(move || Message::Editing {
                    id,
                    is_editing: true,
                }),
                entry.content.clone(),
            ),
            html::button(html::on_click(move || Message::Remove { id }), ()),
        ),
    )
}

pub struct App {
    entries: Vec<Entry>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            entries: vec![Entry {
                id: 0,
                content: String::from("A"),
                is_done: true,
                is_editing: true,
            }],
        }
    }
}

impl Model<Message> for App {
    fn handle(&mut self, msg: Message) -> ControlFlow {
        match msg {
            Message::Check { id, is_checked } => {
                if let Some(entry) = self.entries.iter_mut().find(|entry| entry.id == id) {
                    entry.is_done = is_checked;
                }
            }
            Message::Editing { id, is_editing } => {
                if let Some(entry) = self.entries.iter_mut().find(|entry| entry.id == id) {
                    entry.is_editing = is_editing;
                }
            }
            Message::Remove { id } => {
                if let Some(idx) = self.entries.iter().position(|entry| entry.id == id) {
                    self.entries.remove(idx);
                }
            }
        }

        ControlFlow::Rebuild
    }
}

fn main() {
    web::run(App::default(), view)
}
