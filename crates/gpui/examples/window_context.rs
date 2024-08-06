use gpui::*;

struct Main {
    focus_handle: FocusHandle
}

impl Main {
    fn backspace(&mut self, _: &Backspace, cx: &mut ViewContext<Self>) {
        println!("{}", cx.window_context().is_window_active());
    }
}


impl Render for Main {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .size_full()
            .track_focus(&self.focus_handle)
            .bg(rgb(0x2e7d32))
            .key_context("TextInput")
            .on_action(cx.listener(Self::backspace))
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.bind_keys([
            KeyBinding::new("backspace", Backspace, None),
        ]);

        let bounds = Bounds::centered(None, size(px(300.0), px(300.0)), cx);
        cx
            .open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |cx| {
                    cx.new_view(|cx| Main {
                        focus_handle: cx.focus_handle()
                    })
                },
            )
            .unwrap();
    });
}

actions!(action, [Backspace]);
