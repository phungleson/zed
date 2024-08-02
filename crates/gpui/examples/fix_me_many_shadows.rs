use derive_more::Into;
use smallvec::smallvec;
use gpui::*;

pub fn box_shadow(
    x: impl Into<Pixels>,
    y: impl Into<Pixels>,
    blur_radius: impl Into<Pixels>,
    spread_radius: impl Into<Pixels>,
    color: impl Into<Hsla>,
) -> BoxShadow {
    BoxShadow {
        offset: point(x.into(), y.into()),
        blur_radius: blur_radius.into(),
        spread_radius: spread_radius.into(),
        color: color.into(),
    }
}

#[derive(IntoElement)]
struct Logo {
}

impl RenderOnce for Logo {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let color = rgb(0x4566ae);
        let white = rgb(0xffffff);

        div()
            .size_4()
            .child(div().size(px(1.)).shadow(smallvec![
                //
                box_shadow(2.0, 1.0, 0.1, 0.0, color),
                box_shadow(3.0, 1.0, 0.1, 0.0, color),
                box_shadow(4.0, 1.0, 0.1, 0.0, color),
                box_shadow(5.0, 1.0, 0.1, 0.0, color),
                box_shadow(6.0, 1.0, 0.1, 0.0, color),
                box_shadow(7.0, 1.0, 0.1, 0.0, color),
                box_shadow(8.0, 1.0, 0.1, 0.0, color),
                box_shadow(9.0, 1.0, 0.1, 0.0, color),
                box_shadow(10.0, 1.0, 0.1, 0.0, color),
                box_shadow(11.0, 1.0, 0.1, 0.0, color),
                box_shadow(12.0, 1.0, 0.1, 0.0, color),
                box_shadow(13.0, 1.0, 0.1, 0.0, color),
                box_shadow(14.0, 1.0, 0.1, 0.0, color),
                box_shadow(15.0, 1.0, 0.1, 0.0, color),
                //
                box_shadow(1.0, 2.0, 0.1, 0.0, color),
                box_shadow(2.0, 2.0, 0.1, 0.0, color),
                box_shadow(3.0, 2.0, 0.1, 0.0, color),
                box_shadow(4.0, 2.0, 0.1, 0.0, color),
                box_shadow(5.0, 2.0, 0.1, 0.0, color),
                box_shadow(6.0, 2.0, 0.1, 0.0, color),
                box_shadow(7.0, 2.0, 0.1, 0.0, color),
                box_shadow(8.0, 2.0, 0.1, 0.0, color),
                box_shadow(9.0, 2.0, 0.1, 0.0, color),
                box_shadow(10.0, 2.0, 0.1, 0.0, color),
                box_shadow(11.0, 2.0, 0.1, 0.0, white),
                box_shadow(12.0, 2.0, 0.1, 0.0, white),
                box_shadow(13.0, 2.0, 0.1, 0.0, white),
                box_shadow(14.0, 2.0, 0.1, 0.0, white),
                box_shadow(15.0, 2.0, 0.1, 0.0, color),
                box_shadow(16.0, 2.0, 0.1, 0.0, color),
                //
                box_shadow(1.0, 3.0, 0.1, 0.0, color),
                box_shadow(2.0, 3.0, 0.1, 0.0, color),
                box_shadow(3.0, 3.0, 0.1, 0.0, color),
                box_shadow(4.0, 3.0, 0.1, 0.0, color),
                box_shadow(5.0, 3.0, 0.1, 0.0, color),
                box_shadow(6.0, 3.0, 0.1, 0.0, color),
                box_shadow(7.0, 3.0, 0.1, 0.0, color),
                box_shadow(8.0, 3.0, 0.1, 0.0, color),
                box_shadow(9.0, 3.0, 0.1, 0.0, color),
                box_shadow(10.0, 3.0, 0.1, 0.0, white),
                box_shadow(11.0, 3.0, 0.1, 0.0, white),
                box_shadow(12.0, 3.0, 0.1, 0.0, white),
                box_shadow(13.0, 3.0, 0.1, 0.0, white),
                box_shadow(14.0, 3.0, 0.1, 0.0, white),
                box_shadow(15.0, 3.0, 0.1, 0.0, color),
                box_shadow(16.0, 3.0, 0.1, 0.0, color),
                //
                box_shadow(1.0, 4.0, 0.1, 0.0, color),
                box_shadow(2.0, 4.0, 0.1, 0.0, color),
                box_shadow(3.0, 4.0, 0.1, 0.0, color),
                box_shadow(4.0, 4.0, 0.1, 0.0, color),
                box_shadow(5.0, 4.0, 0.1, 0.0, color),
                box_shadow(6.0, 4.0, 0.1, 0.0, color),
                box_shadow(7.0, 4.0, 0.1, 0.0, color),
                box_shadow(8.0, 4.0, 0.1, 0.0, color),
                box_shadow(9.0, 4.0, 0.1, 0.0, white),
                box_shadow(10.0, 4.0, 0.1, 0.0, white),
                box_shadow(11.0, 4.0, 0.1, 0.0, white),
                box_shadow(12.0, 4.0, 0.1, 0.0, white),
                box_shadow(13.0, 4.0, 0.1, 0.0, white),
                box_shadow(14.0, 4.0, 0.1, 0.0, white),
                box_shadow(15.0, 4.0, 0.1, 0.0, color),
                box_shadow(16.0, 4.0, 0.1, 0.0, color),
                //
                box_shadow(1.0, 5.0, 0.1, 0.0, color),
                box_shadow(2.0, 5.0, 0.1, 0.0, color),
                box_shadow(3.0, 5.0, 0.1, 0.0, color),
                box_shadow(4.0, 5.0, 0.1, 0.0, color),
                box_shadow(5.0, 5.0, 0.1, 0.0, color),
                box_shadow(6.0, 5.0, 0.1, 0.0, color),
                box_shadow(7.0, 5.0, 0.1, 0.0, color),
                box_shadow(8.0, 5.0, 0.1, 0.0, color),
                box_shadow(9.0, 5.0, 0.1, 0.0, white),
                box_shadow(10.0, 5.0, 0.1, 0.0, white),
                box_shadow(11.0, 5.0, 0.1, 0.0, white),
                box_shadow(12.0, 5.0, 0.1, 0.0, color),
                box_shadow(13.0, 5.0, 0.1, 0.0, color),
                box_shadow(14.0, 5.0, 0.1, 0.0, color),
                box_shadow(15.0, 5.0, 0.1, 0.0, color),
                box_shadow(16.0, 5.0, 0.1, 0.0, color),
                //
                box_shadow(1.0, 6.0, 0.1, 0.0, color),
                box_shadow(2.0, 6.0, 0.1, 0.0, color),
                box_shadow(3.0, 6.0, 0.1, 0.0, color),
                box_shadow(4.0, 6.0, 0.1, 0.0, color),
                box_shadow(5.0, 6.0, 0.1, 0.0, color),
                box_shadow(6.0, 6.0, 0.1, 0.0, color),
                box_shadow(7.0, 6.0, 0.1, 0.0, color),
                box_shadow(8.0, 6.0, 0.1, 0.0, color),
                box_shadow(9.0, 6.0, 0.1, 0.0, white),
                box_shadow(10.0, 6.0, 0.1, 0.0, white),
                box_shadow(11.0, 6.0, 0.1, 0.0, white),
                box_shadow(12.0, 6.0, 0.1, 0.0, color),
                box_shadow(13.0, 6.0, 0.1, 0.0, color),
                box_shadow(14.0, 6.0, 0.1, 0.0, color),
                box_shadow(15.0, 6.0, 0.1, 0.0, color),
                box_shadow(16.0, 6.0, 0.1, 0.0, color),
                //
                box_shadow(1.0, 7.0, 0.1, 0.0, color),
                box_shadow(2.0, 7.0, 0.1, 0.0, color),
                box_shadow(3.0, 7.0, 0.1, 0.0, color),
                box_shadow(4.0, 7.0, 0.1, 0.0, color),
                box_shadow(5.0, 7.0, 0.1, 0.0, color),
                box_shadow(6.0, 7.0, 0.1, 0.0, color),
                box_shadow(7.0, 7.0, 0.1, 0.0, color),
                box_shadow(8.0, 7.0, 0.1, 0.0, color),
                box_shadow(9.0, 7.0, 0.1, 0.0, white),
                box_shadow(10.0, 7.0, 0.1, 0.0, white),
                box_shadow(11.0, 7.0, 0.1, 0.0, white),
                box_shadow(12.0, 7.0, 0.1, 0.0, color),
                box_shadow(13.0, 7.0, 0.1, 0.0, color),
                box_shadow(14.0, 7.0, 0.1, 0.0, color),
                box_shadow(15.0, 7.0, 0.1, 0.0, color),
                box_shadow(16.0, 7.0, 0.1, 0.0, color),
                //
                box_shadow(1.0, 8.0, 0.1, 0.0, color),
                box_shadow(2.0, 8.0, 0.1, 0.0, color),
                box_shadow(3.0, 8.0, 0.1, 0.0, color),
                box_shadow(4.0, 8.0, 0.1, 0.0, color),
                box_shadow(5.0, 8.0, 0.1, 0.0, color),
                box_shadow(6.0, 8.0, 0.1, 0.0, white),
                box_shadow(7.0, 8.0, 0.1, 0.0, white),
                box_shadow(8.0, 8.0, 0.1, 0.0, white),
                box_shadow(9.0, 8.0, 0.1, 0.0, white),
                box_shadow(10.0, 8.0, 0.1, 0.0, white),
                box_shadow(11.0, 8.0, 0.1, 0.0, white),
                box_shadow(12.0, 8.0, 0.1, 0.0, white),
                box_shadow(13.0, 8.0, 0.1, 0.0, white),
                box_shadow(14.0, 8.0, 0.1, 0.0, white),
                box_shadow(15.0, 8.0, 0.1, 0.0, color),
                box_shadow(16.0, 8.0, 0.1, 0.0, color),
                //
                box_shadow(1.0, 9.0, 0.1, 0.0, color),
                box_shadow(2.0, 9.0, 0.1, 0.0, color),
                box_shadow(3.0, 9.0, 0.1, 0.0, color),
                box_shadow(4.0, 9.0, 0.1, 0.0, color),
                box_shadow(5.0, 9.0, 0.1, 0.0, color),
                box_shadow(6.0, 9.0, 0.1, 0.0, white),
                box_shadow(7.0, 9.0, 0.1, 0.0, white),
                box_shadow(8.0, 9.0, 0.1, 0.0, white),
                box_shadow(9.0, 9.0, 0.1, 0.0, white),
                box_shadow(10.0, 9.0, 0.1, 0.0, white),
                box_shadow(11.0, 9.0, 0.1, 0.0, white),
                box_shadow(12.0, 9.0, 0.1, 0.0, white),
                box_shadow(13.0, 9.0, 0.1, 0.0, white),
                box_shadow(14.0, 9.0, 0.1, 0.0, white),
                box_shadow(15.0, 9.0, 0.1, 0.0, color),
                box_shadow(16.0, 9.0, 0.1, 0.0, color),
                //
                box_shadow(1.0, 10.0, 0.1, 0.0, color),
                box_shadow(2.0, 10.0, 0.1, 0.0, color),
                box_shadow(3.0, 10.0, 0.1, 0.0, color),
                box_shadow(4.0, 10.0, 0.1, 0.0, color),
                box_shadow(5.0, 10.0, 0.1, 0.0, color),
                box_shadow(6.0, 10.0, 0.1, 0.0, white),
                box_shadow(7.0, 10.0, 0.1, 0.0, white),
                box_shadow(8.0, 10.0, 0.1, 0.0, white),
                box_shadow(9.0, 10.0, 0.1, 0.0, white),
                box_shadow(10.0, 10.0, 0.1, 0.0, white),
                box_shadow(11.0, 10.0, 0.1, 0.0, white),
                box_shadow(12.0, 10.0, 0.1, 0.0, white),
                box_shadow(13.0, 10.0, 0.1, 0.0, white),
                box_shadow(14.0, 10.0, 0.1, 0.0, white),
                box_shadow(15.0, 10.0, 0.1, 0.0, color),
                box_shadow(16.0, 10.0, 0.1, 0.0, color),
                //
                box_shadow(1.0, 11.0, 0.1, 0.0, color),
                box_shadow(2.0, 11.0, 0.1, 0.0, color),
                box_shadow(3.0, 11.0, 0.1, 0.0, color),
                box_shadow(4.0, 11.0, 0.1, 0.0, color),
                box_shadow(5.0, 11.0, 0.1, 0.0, color),
                box_shadow(6.0, 11.0, 0.1, 0.0, color),
                box_shadow(7.0, 11.0, 0.1, 0.0, color),
                box_shadow(8.0, 11.0, 0.1, 0.0, color),
                box_shadow(9.0, 11.0, 0.1, 0.0, white),
                box_shadow(10.0, 11.0, 0.1, 0.0, white),
                box_shadow(11.0, 11.0, 0.1, 0.0, white),
                box_shadow(12.0, 11.0, 0.1, 0.0, color),
                box_shadow(13.0, 11.0, 0.1, 0.0, color),
                box_shadow(14.0, 11.0, 0.1, 0.0, color),
                box_shadow(15.0, 11.0, 0.1, 0.0, color),
                box_shadow(16.0, 11.0, 0.1, 0.0, color),
                //
                box_shadow(1.0, 12.0, 0.1, 0.0, color),
                box_shadow(2.0, 12.0, 0.1, 0.0, color),
                box_shadow(3.0, 12.0, 0.1, 0.0, color),
                box_shadow(4.0, 12.0, 0.1, 0.0, color),
                box_shadow(5.0, 12.0, 0.1, 0.0, color),
                box_shadow(6.0, 12.0, 0.1, 0.0, color),
                box_shadow(7.0, 12.0, 0.1, 0.0, color),
                box_shadow(8.0, 12.0, 0.1, 0.0, color),
                box_shadow(9.0, 12.0, 0.1, 0.0, white),
                box_shadow(10.0, 12.0, 0.1, 0.0, white),
                box_shadow(11.0, 12.0, 0.1, 0.0, white),
                box_shadow(12.0, 12.0, 0.1, 0.0, color),
                box_shadow(13.0, 12.0, 0.1, 0.0, color),
                box_shadow(14.0, 12.0, 0.1, 0.0, color),
                box_shadow(15.0, 12.0, 0.1, 0.0, color),
                box_shadow(16.0, 12.0, 0.1, 0.0, color),
                //
                box_shadow(1.0, 13.0, 0.1, 0.0, color),
                box_shadow(2.0, 13.0, 0.1, 0.0, color),
                box_shadow(3.0, 13.0, 0.1, 0.0, color),
                box_shadow(4.0, 13.0, 0.1, 0.0, color),
                box_shadow(5.0, 13.0, 0.1, 0.0, color),
                box_shadow(6.0, 13.0, 0.1, 0.0, color),
                box_shadow(7.0, 13.0, 0.1, 0.0, color),
                box_shadow(8.0, 13.0, 0.1, 0.0, color),
                box_shadow(9.0, 13.0, 0.1, 0.0, white),
                box_shadow(10.0, 13.0, 0.1, 0.0, white),
                box_shadow(11.0, 13.0, 0.1, 0.0, white),
                box_shadow(12.0, 13.0, 0.1, 0.0, color),
                box_shadow(13.0, 13.0, 0.1, 0.0, color),
                box_shadow(14.0, 13.0, 0.1, 0.0, color),
                box_shadow(15.0, 13.0, 0.1, 0.0, color),
                box_shadow(16.0, 13.0, 0.1, 0.0, color),
                //
                box_shadow(1.0, 14.0, 0.1, 0.0, color),
                box_shadow(2.0, 14.0, 0.1, 0.0, color),
                box_shadow(3.0, 14.0, 0.1, 0.0, color),
                box_shadow(4.0, 14.0, 0.1, 0.0, color),
                box_shadow(5.0, 14.0, 0.1, 0.0, color),
                box_shadow(6.0, 14.0, 0.1, 0.0, color),
                box_shadow(7.0, 14.0, 0.1, 0.0, color),
                box_shadow(8.0, 14.0, 0.1, 0.0, color),
                box_shadow(9.0, 14.0, 0.1, 0.0, white),
                box_shadow(10.0, 14.0, 0.1, 0.0, white),
                box_shadow(11.0, 14.0, 0.1, 0.0, white),
                box_shadow(12.0, 14.0, 0.1, 0.0, color),
                box_shadow(13.0, 14.0, 0.1, 0.0, color),
                box_shadow(14.0, 14.0, 0.1, 0.0, color),
                box_shadow(15.0, 14.0, 0.1, 0.0, color),
                box_shadow(16.0, 14.0, 0.1, 0.0, color),
                //
                box_shadow(1.0, 15.0, 0.1, 0.0, color),
                box_shadow(2.0, 15.0, 0.1, 0.0, color),
                box_shadow(3.0, 15.0, 0.1, 0.0, color),
                box_shadow(4.0, 15.0, 0.1, 0.0, color),
                box_shadow(5.0, 15.0, 0.1, 0.0, color),
                box_shadow(6.0, 15.0, 0.1, 0.0, color),
                box_shadow(7.0, 15.0, 0.1, 0.0, color),
                box_shadow(8.0, 15.0, 0.1, 0.0, color),
                box_shadow(9.0, 15.0, 0.1, 0.0, white),
                box_shadow(10.0, 15.0, 0.1, 0.0, white),
                box_shadow(11.0, 15.0, 0.1, 0.0, white),
                box_shadow(12.0, 15.0, 0.1, 0.0, color),
                box_shadow(13.0, 15.0, 0.1, 0.0, color),
                box_shadow(14.0, 15.0, 0.1, 0.0, color),
                box_shadow(15.0, 15.0, 0.1, 0.0, color),
                box_shadow(16.0, 15.0, 0.1, 0.0, color),
                //
                box_shadow(2.0, 16.0, 0.1, 0.0, color),
                box_shadow(3.0, 16.0, 0.1, 0.0, color),
                box_shadow(4.0, 16.0, 0.1, 0.0, color),
                box_shadow(5.0, 16.0, 0.1, 0.0, color),
                box_shadow(6.0, 16.0, 0.1, 0.0, color),
                box_shadow(7.0, 16.0, 0.1, 0.0, color),
                box_shadow(8.0, 16.0, 0.1, 0.0, color),
                box_shadow(9.0, 16.0, 0.1, 0.0, white),
                box_shadow(10.0, 16.0, 0.1, 0.0, white),
                box_shadow(11.0, 16.0, 0.1, 0.0, white),
                box_shadow(12.0, 16.0, 0.1, 0.0, color),
                box_shadow(13.0, 16.0, 0.1, 0.0, color),
                box_shadow(14.0, 16.0, 0.1, 0.0, color),
                box_shadow(15.0, 16.0, 0.1, 0.0, color),
            ]))
    }
}

struct ManyShadows {
}

impl Render for ManyShadows {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .size_full()
            .gap_4()
            .child(Logo {})
            .child(Logo {})
            .child(Logo {})
            .child(Logo {})
            .child(Logo {})
            .child(Logo {})
            .child(Logo {})
            .child(Logo {})
            .child(Logo {})
            .child(Logo {})
            .child(Logo {})
            .child(Logo {})
            .child(Logo {})
            .child(Logo {})
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
            |cx| {
                cx.new_view(|_cx| ManyShadows {
                })
            },
        )
        .unwrap();
    });
}
