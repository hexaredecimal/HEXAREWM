use penrose::{
    builtin::{
        actions::{exit, log_current_state, modify_with, send_layout_message, spawn},
        layout::messages::{ExpandMain, IncMain, ShrinkMain},
    },
    core::bindings::KeyEventHandler,
    map,
    x11rb::RustConn,
};
use std::collections::HashMap;
const ROFI: &'static str = "rofi -show drun";

pub fn raw_key_bindings() -> HashMap<String, Box<dyn KeyEventHandler<RustConn>>> {
    let mut raw_bindings = map! {
        map_keys: |k: &str| k.to_owned();

        "M-Down" => modify_with(|cs| cs.focus_down()),
        "M-Up" => modify_with(|cs| cs.focus_up()),
        "M-Left" => modify_with(|cs| cs.swap_down()),
        "M-Right" => modify_with(|cs| cs.swap_up()),
        "M-S-q" => modify_with(|cs| cs.kill_focused()),
        "M-Tab" => modify_with(|cs| cs.toggle_tag()),
        "M-bracketright" => modify_with(|cs| cs.next_screen()),
        "M-bracketleft" => modify_with(|cs| cs.previous_screen()),
        "M-n" => modify_with(|cs| cs.next_layout()),
        "M-p" => modify_with(|cs| cs.previous_layout()),
        "M-S-Up" => send_layout_message(|| IncMain(1)),
        "M-S-Down" => send_layout_message(|| IncMain(-1)),
        "M-S-Right" => send_layout_message(|| ExpandMain),
        "M-S-Left" => send_layout_message(|| ShrinkMain),
        "M-d" => spawn("dmenu_run"),
        "M-m" => spawn(ROFI),
        "M-S-s" => log_current_state(),
        "M-Return" => spawn("alacritty"),
        "M-S-e" => exit(),
    };

    for tag in &["1", "2", "3", "4", "5", "6", "7", "8", "9"] {
        raw_bindings.extend([
            (
                format!("M-{tag}"),
                modify_with(move |client_set| client_set.focus_tag(tag)),
            ),
            (
                format!("M-S-{tag}"),
                modify_with(move |client_set| client_set.move_focused_to_tag(tag)),
            ),
        ]);
    }

    raw_bindings
}
