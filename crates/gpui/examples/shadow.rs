use gpui::*;
use smallvec::smallvec;

struct Shadow {}

impl Render for Shadow {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0xffffff))
            .size_full()
            .justify_center()
            .items_center()
            .size(Length::Definite(Pixels(300.0).into()))
            .justify_center()
            .items_center()
            .gap_4()
            .child(div().size_8().shadow_sm())
            .child(
                div()
                    .size_4()
                    .rounded_full()
                    .bg(rgb(0x000000))
                    .shadow(smallvec![
                        BoxShadow {
                            color: hsla(0., 0., 0., 1.0),
                            offset: point(px(24.), px(24.)),
                            blur_radius: px(0.0),
                            spread_radius: px(8.),
                            inset: false,
                        },
                        BoxShadow {
                            color: hsla(0., 0., 100., 1.0),
                            offset: point(px(24.), px(24.)),
                            blur_radius: px(0.0),
                            spread_radius: px(0.),
                            inset: false,
                        },
                    ]),
            )
            .child(
                div()
                    .size_4()
                    .rounded_full()
                    .bg(rgb(0xff0000))
                    .shadow(smallvec![
                        BoxShadow {
                            color: rgb(0xff0000).into(),
                            offset: point(px(16.), px(16.)),
                            blur_radius: px(1.0),
                            spread_radius: px(0.),
                            inset: false,
                        },
                        BoxShadow {
                            color: rgb(0xff0000).into(),
                            offset: point(px(32.), px(32.)),
                            blur_radius: px(0.0),
                            spread_radius: px(0.),
                            inset: false,
                        },
                    ]),
            )
            .child(
                div()
                    .size_4()
                    .rounded_tl(px(8.))
                    .rounded_br(px(8.))
                    .bg(rgb(0x0000ff))
                    .shadow(smallvec![BoxShadow {
                        color: rgb(0x0000ff).into(),
                        offset: point(px(32.), px(32.)),
                        blur_radius: px(0.0),
                        spread_radius: px(0.),
                        inset: false,
                    }]),
            )
            .child(
                div()
                    .h_2()
                    .w_16()
                    .rounded_full()
                    .bg(rgb(0x00ff00))
                    .shadow(smallvec![BoxShadow {
                        color: rgb(0x00ff00).into(),
                        offset: point(px(32.), px(32.)),
                        blur_radius: px(8.),
                        spread_radius: px(0.),
                        inset: false,
                    }]),
            )
            .child(div().size_4().rounded_full().shadow(smallvec![
                BoxShadow {
                    color: rgb(0x00ff00).into(),
                    offset: point(px(-8.), px(-8.)),
                    blur_radius: px(0.),
                    spread_radius: px(0.),
                    inset: false,
                },
                BoxShadow {
                    color: rgb(0x00ff00).into(),
                    offset: point(px(8.), px(8.)),
                    blur_radius: px(8.),
                    spread_radius: px(0.),
                    inset: false,
                }
            ]))
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        let bounds = Bounds::centered(None, size(px(300.0), px(300.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |cx| cx.new_view(|_cx| Shadow {}),
        )
        .unwrap();
    });
}
