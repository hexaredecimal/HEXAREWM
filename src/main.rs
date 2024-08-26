use penrose::{
    core::{bindings::parse_keybindings_with_xmodmap, Config, WindowManager},
    extensions::hooks::add_ewmh_hooks,
    x11rb::RustConn,
    Result,
};

use penrose_ui::bar::Position;
use std::collections::HashMap;

use hexawm::{
    keybinds::raw_key_bindings, layouts::WmLayout, statusbar::WmStatusBar, wmconfig::WmConfig,
};

const BLACK: u32 = 0x282828ff;
const GREY: u32 = 0x3c3836ff;

fn main() -> Result<()> {
    let config = add_ewmh_hooks(Config {
        default_layouts: WmLayout::layouts(),
        ..Config::default()
    });

    let conn = RustConn::new()?;
    let key_bindings = parse_keybindings_with_xmodmap(raw_key_bindings())?;
    let wm_config = WmConfig::default();

    let bar = WmStatusBar::status_bar(&wm_config, BLACK, GREY, Position::Top);
    let wm = bar.add_to(WindowManager::new(
        config,
        key_bindings,
        HashMap::new(),
        conn,
    )?);

    wm.run()
}
