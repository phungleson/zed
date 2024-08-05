use gpui::*;
use smallvec::smallvec;

struct Shadow {}

impl Render for Shadow {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(hsla(0., 1., 1., 1.))
            .size(Length::Definite(Pixels(300.0).into()))
            .justify_center()
            .items_center()
            .gap_4()
            .child(
                div()
                    .size_4()
                    .rounded_full()
                    .bg(hsla(0., 0., 0., 1.))
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
                    .bg(hsla(0., 1., 0.5, 1.))
                    .shadow(smallvec![
                        BoxShadow {
                            color: hsla(0., 1., 0.5, 1.),
                            offset: point(px(16.), px(16.)),
                            blur_radius: px(1.0),
                            spread_radius: px(0.),
                            inset: false,
                        },
                        BoxShadow {
                            color: hsla(0., 1., 0.5, 1.),
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
                    .bg(hsla(180. / 360., 1., 0.5, 1.))
                    .shadow(smallvec![BoxShadow {
                        color: hsla(180. / 360., 1., 0.5, 1.),
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
                        color: hsla(128. / 360., 1., 0.5, 1.),
                        offset: point(px(32.), px(32.)),
                        blur_radius: px(8.),
                        spread_radius: px(0.),
                        inset: false,
                    }]),
            )
            .child(div().size_4().rounded_full().shadow(smallvec![
                BoxShadow {
                    color: hsla(60. / 360., 1., 0.5, 1.),
                    offset: point(px(-8.), px(-8.)),
                    blur_radius: px(0.),
                    spread_radius: px(0.),
                    inset: false,
                },
                BoxShadow {
                    color: hsla(60. / 360., 1., 0.5, 1.),
                    offset: point(px(8.), px(8.)),
                    blur_radius: px(8.),
                    spread_radius: px(0.),
                    inset: false,
                }
            ]))
            .child(div().size_4().rounded_full().shadow(smallvec![BoxShadow {
                color: hsla(30. / 360., 1., 0.5, 1.),
                offset: point(px(8.), px(8.)),
                blur_radius: px(0.),
                spread_radius: px(0.),
                inset: true,
            },]))
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
