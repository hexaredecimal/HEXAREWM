use hexawm::{
    colordefs::WmColors, keybinds::raw_key_bindings, layouts::WmLayout, statusbar::WmStatusBar,
    wmconfig::WmConfig,
};
use penrose::{
    core::{bindings::parse_keybindings_with_xmodmap, Config, WindowManager},
    extensions::hooks::add_ewmh_hooks,
    x11rb::RustConn,
    Result,
};
use penrose_ui::bar::Position;
use std::collections::HashMap;

fn main() -> Result<()> {
    let config = add_ewmh_hooks(Config {
        default_layouts: WmLayout::layouts(),
        ..Config::default()
    });

    let conn = RustConn::new()?;
    let key_bindings = parse_keybindings_with_xmodmap(raw_key_bindings())?;

    let bar = WmStatusBar::status_bar(
        &mut WmConfig::default(),
        WmColors::black(),
        WmColors::grey(),
        Position::Top,
    );
    let wm = bar.add_to(WindowManager::new(
        config,
        key_bindings,
        HashMap::new(),
        conn,
    )?);
    wm.run()
}
