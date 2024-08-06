use gpui::*;

struct Main {
    focus_handle: FocusHandle
}

impl Main {
    // fn backspace(&mut self, _: &Backspace, cx: &mut ViewContext<Self>) {
    //     if self.selected_range.is_empty() {
    //         self.select_to(self.previous_boundary(self.cursor_offset()), cx)
    //     }
    //     self.replace_text_in_range(None, "", cx)
    // }

    fn backspace(&mut self, _: &Backspace, cx: &mut ViewContext<Self>) {
        println!("{}", cx.window_context().is_window_active());
    }
}

impl FocusableView for Main {
    fn focus_handle(&self, _: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for Main {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
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
        let window = cx
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
