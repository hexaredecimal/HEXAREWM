use penrose::util::{spawn, spawn_for_output_with_args};
use penrose::{x::XConn, Color};
use penrose_ui::bar::widgets::RefreshText;
use penrose_ui::bar::widgets::*;
use penrose_ui::bar::{Position, StatusBar};
use penrose_ui::core::TextStyle;

use crate::colordefs::WmColors;
use crate::wedge::Wedge;
use crate::wmconfig::WmConfig;

pub struct WmStatusBar;

impl WmStatusBar {
    pub fn status_bar<X: XConn>(
        wm: &WmConfig,
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
                Box::new(Self::battery(wm.text_style)),
                Box::new(wifi_network(wm.text_style)),
                Box::new(amixer_volume("Master", wm.text_style)),
                Box::new(current_date_and_time(wm.text_style)),
                Box::new(space),
                Box::new(Wedge::start(WmColors::white(), WmColors::black())),
            ],
        )
        .unwrap()
    }

    pub fn battery(style: TextStyle) -> RefreshText {
        let percent = spawn_for_output_with_args("cat", &["/sys/class/power_supply/BAT0/capacity"])
            .unwrap_or_default();
        let percent = percent.clone();
        let trim = percent.trim();
        let level = trim.parse::<i32>().unwrap_or_default();

        let mut battery_style = style.clone();
        // TODO: Set this value from the config
        // TODO: Trigger the noticications only once
        // Move the batter_level info to the wm, then mutate it and run once
        let (fg, text) = if level < 20 {
            spawn(
                "notify-send --urgency=critical -t 5000 'Low Battery Level' --icon=dialog-information",
            ).unwrap();
            (WmColors::red(), format!("Danger: {percent}"))
        } else if level <= 50 {
            spawn(
                "notify-send --urgency=critical -t 5000 'Battery Level Warning' --icon=dialog-information",
            ).unwrap();
            (WmColors::orange(), format!("Warning: {percent}"))
        } else {
            (WmColors::green(), format!("{percent}"))
        };

        // notify-send 'Hello world!' 'This is an example notification.' --icon=dialog-information
        battery_style.fg = fg.into();
        RefreshText::new(battery_style, move || text.clone())
    }
}
