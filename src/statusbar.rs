use penrose::util::spawn_for_output_with_args;
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
                Box::new(battery_summary("BAT: ", wm.text_style)),
                Box::new(wifi_network(wm.text_style)),
                Box::new(amixer_volume("Master", wm.text_style)),
                Box::new(current_date_and_time(wm.text_style)),
                Box::new(Self::user_text(wm.text_style)),
                Box::new(space),
                Box::new(Wedge::start(WmColors::white(), WmColors::black())),
            ],
        )
        .unwrap()
    }

    pub fn user_text(style: TextStyle) -> RefreshText {
        RefreshText::new(style, || {
            spawn_for_output_with_args("uname", &["-m -o"])
                .unwrap_or_default()
                .trim()
                .to_string()
        })
    }
}
