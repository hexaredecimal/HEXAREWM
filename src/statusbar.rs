//use penrose::util::spawn;
use penrose::{x::XConn, Color};
use penrose_ui::bar::widgets::RefreshText;
use penrose_ui::bar::widgets::*;
use penrose_ui::bar::{Position, StatusBar};
use penrose_ui::core::TextStyle;

use crate::colordefs::WmColors;
use crate::wedge::Wedge;
use crate::wmconfig::WmConfig;
use std::fs;
pub struct WmStatusBar;

impl WmStatusBar {
    pub fn status_bar<X: XConn>(
        wm: &mut WmConfig,
        highlight: impl Into<Color>,
        empty_ws: impl Into<Color>,
        position: Position,
    ) -> StatusBar<X> {
        let max_active_window_chars = 80;
        let highlight = highlight.into();
        let style = wm.text_style;
        let wm_name_text = Text::new(" HEXAREWM ", style, false, true);
        let space = Spacer::new(vec![], 0.5);

        StatusBar::try_new(
            position,
            wm.bar_height_px,
            style.bg.unwrap_or_else(|| 0x000000.into()),
            wm.font,
            wm.point_size,
            vec![
                Box::new(Wedge::start(WmColors::white(), WmColors::black())),
                Box::new(wm_name_text),
                Box::new(Wedge::end(WmColors::white(), WmColors::black())),
                Box::new(Workspaces::new(style, highlight, empty_ws)),
                Box::new(Wedge::start(WmColors::white(), WmColors::black())),
                Box::new(CurrentLayout::new(style)),
                Box::new(ActiveWindowName::new(
                    max_active_window_chars,
                    TextStyle {
                        bg: Some(highlight),
                        padding: (6, 4),
                        ..style
                    },
                    true,
                    true,
                )),
                Box::new(Wedge::start(WmColors::white(), WmColors::black())),
                //Box::new(battery_summary("BAT: ", wm.text_style)),
                Box::new(Self::battery(wm)),
                Box::new(wifi_network(wm.text_style)),
                Box::new(amixer_volume("Master", wm.text_style)),
                Box::new(current_date_and_time(wm.text_style)),
                Box::new(space),
                Box::new(Wedge::start(WmColors::white(), WmColors::black())),
            ],
        )
        .unwrap()
    }

    pub fn battery(wm: &mut WmConfig) -> RefreshText {
        let mut battery_style = wm.text_style;
        // TODO: Set this value from the config
        // TODO: Trigger the noticications only once
        // Move the batter_level info to the wm, then mutate it and run once
        let (status, color) = Self::battery_text(wm).unwrap();
        let text = status.unwrap_or(0.to_string());

        // TODO: Show a notification when the Battery level is 50 && 20
        // if wm.battery == 50 {
        //    spawn("notify-send --urgency=critical -t 5000 'Low Battery Level' --icon=dialog-information").unwrap();
        //}

        battery_style.fg = color.into();
        RefreshText::new(battery_style, move || text.clone())
    }

    fn battery_text(wm: &mut WmConfig) -> Option<(Option<String>, u32)> {
        let status = Self::read_sys_file("BAT0", "status")?;
        let energy_now: u32 = Self::read_sys_file("BAT0", "energy_now")?.parse().ok()?;
        let energy_full: u32 = Self::read_sys_file("BAT0", "energy_full")?.parse().ok()?;

        let charge = energy_now * 100 / energy_full;
        wm.battery = charge as i32;

        let icon = if status == "Charging" {
            ""
        } else if charge >= 90 || status == "Full" {
            ""
        } else if charge >= 70 {
            ""
        } else if charge >= 50 {
            ""
        } else if charge >= 20 {
            ""
        } else {
            ""
        };

        let color = if charge < 20 {
            //Critical Level
            WmColors::red()
        } else if charge < 50 {
            WmColors::orange()
        } else {
            WmColors::white()
        };

        Some((Some(format!(" {icon} {charge}% - {status} ")), color))
    }

    fn read_sys_file(bat: &str, fname: &str) -> Option<String> {
        fs::read_to_string(format!("/sys/class/power_supply/{bat}/{fname}"))
            .ok()
            .map(|s| s.trim().to_string())
    }
}
