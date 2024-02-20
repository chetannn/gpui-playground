use gpui::*;

struct TextMessage {
    text: SharedString,
}

impl Render for TextMessage {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0x4f5165))
            .size(Length::Definite(Pixels(400.0).into()))
            .justify_center()
            .items_center()
            .shadow_lg()
            .border_color(rgb(0xeeeeee))
            .child(
                div()
                    .w_64()
                    .border_2()
                    .rounded_md()
                    .border_color(rgb(0xeeeeee))
                    .child(self.text.clone()),
            )
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(
            WindowOptions {
                bounds: WindowBounds::Fixed(Bounds {
                    origin: Point::new(
                        GlobalPixels::from(500.0).into(),
                        GlobalPixels::from(200.0).into(),
                    ),
                    size: Size {
                        width: GlobalPixels::from(400.0).into(),
                        height: GlobalPixels::from(400.0).into(),
                    },
                }),
                focus: true,
                is_movable: true,
                show: true,
                center: true,
                titlebar: Some(TitlebarOptions {
                    title: Some("Hello".into()),
                    appears_transparent: true,
                    traffic_light_position: Default::default(),
                }),
                display_id: None,
                kind: WindowKind::Normal,
            },
            |cx| {
                cx.new_view(|cx| TextMessage {
                    text: SharedString::from(cx.read_from_clipboard().unwrap().text().to_string()),
                })
            },
        );
    });
}
