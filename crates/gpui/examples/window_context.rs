use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;
use image::codecs::png;
use image::{ExtendedColorType, ImageEncoder};
use gpui::*;

struct Main {
    focus_handle: FocusHandle
}

impl Main {
    fn backspace(&mut self, _: &Backspace, cx: &mut ViewContext<Self>) {
        let screenshot = cx.screenshot();
        println!("screenshot");
        let out_file = PathBuf::from("out.png");
        let file = File::create(&out_file).unwrap();
        let ref mut writer = BufWriter::new(file);

        let encoder = png::PngEncoder::new(writer);
        encoder
            .write_image(
                &screenshot.data,
                screenshot.width,
                screenshot.height,
                ExtendedColorType::Rgba8,
            )
            .unwrap();
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
