use druid::{Color, Env, Key};

pub const PRIMARY_COLOR: Key<Color> = Key::new("druid-example.primary.color");
pub const NEUTRAL_COLOR: Key<Color> = Key::new("druid-example.neutral.color");

pub const MENU_ITEM_TEXT_SIZE: Key<f64> = Key::new("druid-example.menu-item.text-size");
pub const MENU_ITEM_SELECTED_COLOR: Key<Color> = Key::new("druid-example.menu-item.selected.color");
pub const MENU_ITEM_HOT_COLOR: Key<Color> = Key::new("druid-example.menu-item.hot.color");
pub const MENU_ITEM_DEFAULT_COLOR: Key<Color> = Key::new("druid-example.menu-item.default.color");

pub fn init(env: &mut Env) {
    env.set(PRIMARY_COLOR, Color::rgb8(0x4c, 0xd2, 0xff));
    env.set(NEUTRAL_COLOR, Color::rgb8(0xbb, 0xbb, 0xbb));

    env.set(MENU_ITEM_SELECTED_COLOR, env.get(PRIMARY_COLOR));
    env.set(MENU_ITEM_HOT_COLOR, Color::rgb8(0x33, 0xcc, 0xff));
    env.set(MENU_ITEM_DEFAULT_COLOR, env.get(NEUTRAL_COLOR));
    env.set(MENU_ITEM_TEXT_SIZE, 16.0);
}
