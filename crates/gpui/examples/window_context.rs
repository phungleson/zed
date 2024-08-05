use gpui::*;

struct Main {}

impl Render for Main {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div().flex().bg(rgb(0x2e7d32))
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        let bounds = Bounds::centered(None, size(px(300.0), px(300.0)), cx);
        cx.on_action(check_window_context);
        cx.set_menus(vec![Menu {
            name: "Action".into(),
            items: vec![MenuItem::action("CheckWindowContext", CheckWindowContext)],
        }]);

        let window = cx
            .open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |cx| cx.new_view(|_cx| Main {}),
            )
            .unwrap();

        window
            .update(cx, |_, cx| {
                cx.activate(true);
            })
            .unwrap();
    });
}

actions!(action, [CheckWindowContext]);

fn check_window_context(_: &CheckWindowContext, cx: &mut AppContext) {
    // I would like to access WindowContext, the current API seem to expose it only via #update
    cx.active_window()
        .unwrap()
        .update(cx, |_, _| println!("Nothing wrong"))
        .expect("window somehow not found?");
    // This doesn't work either
    // cx.update_window(cx.active_window().unwrap(), |_, _cx| {
    //     println!("Nothing wrong")
    // })
    // .expect("window somehow not found?");
}
