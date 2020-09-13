use druid::{AppLauncher, Size, WindowDesc};

mod data;
mod theme;
mod util;
mod widget;

pub fn main() {
    AppLauncher::with_window(create_main_window())
        .launch(crate::data::AppData::new())
        .expect("Failed to launch application");
}

fn create_main_window() -> WindowDesc<crate::data::AppData> {
    let window_size = Size::new(720.0, 365.0);
    WindowDesc::new(crate::widget::App::new)
        .with_min_size(window_size)
        .window_size(window_size)
        .title("Druid example")
}
