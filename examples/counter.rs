use viewbuilder::{prelude::*, Renderer};

fn app(cx: Scope) -> Element {
    let count = use_state(cx, || 0);

    use_future(cx, count, |count| async move {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            count.set(*count + 1);
        }
    });

    render!(
        view { flex_direction: FlexDirection::Column,
            view { width: 1.percent(), height: 1.percent(), "High five count: {count}" }
            view { flex_direction: FlexDirection::Row,
                view { width: 200.dp(), height: 100.dp(), background_color: Color::from_rgb(0, 255, 255),"Up high!" }
                view { width: 200.dp(), height: 100.dp(), background_color: Color::from_rgb(0, 255, 255), "Down low!" }
            }
        }
    )
}

#[tokio::main]
async fn main() {
    Renderer.run(app);
}
