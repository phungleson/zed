use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;
use image::codecs::png;
use image::{ExtendedColorType, ImageEncoder};
use gpui::*;

struct Main {
    text: SharedString,
}

impl Render for Main {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0x2e7d32))
            .size(Length::Definite(Pixels(300.0).into()))
            .justify_center()
            .items_center()
            .shadow_lg()
            .border_1()
            .border_color(rgb(0x0000ff))
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(format!("{}!", &self.text))
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
                cx.new_view(|_cx| Main {
                    text: "Screenshot".into(),
                })
            },
        )
        .unwrap();

        cx.on_action(screenshot);
        cx.set_menus(vec![Menu {
            name: "screenshot".into(),
            items: vec![MenuItem::action("Screenshot", Screenshot)],
        }]);
    });
}

actions!(screenshot, [Screenshot]);

fn screenshot(_: &Screenshot, cx: &mut AppContext) {
    println!("screenshot");
    println!("window_id: {:?}", cx.windows().first().cloned().unwrap().window_id());
    let window_ids = cx.windows().iter().map(|window| window.window_id()).collect::<Vec<_>>();
    println!("window_ids: {:?}", window_ids);

    cx.active_window().unwrap().update(cx, |_, cx| {
        let screenshot = cx.screenshot();
        println!("screenshot");
        let out_file = PathBuf::from("out.png");
        let file = File::create(&out_file).unwrap();
        let ref mut writer = BufWriter::new(file);

        let encoder = png::PngEncoder::new(writer);
        encoder.write_image(&screenshot.data, screenshot.width, screenshot.height, ExtendedColorType::Rgb8).unwrap();
    }).unwrap();
}