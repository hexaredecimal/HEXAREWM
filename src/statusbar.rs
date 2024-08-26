use penrose::{x::XConn, Color};
use penrose_ui::bar::widgets::*;
use penrose_ui::bar::{Position, StatusBar};
use penrose_ui::core::TextStyle;

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
        let custom_text = Text::new("HEXAREWM", style, false, true);
        let right_pad = Text::new("", style, true, true);

        StatusBar::try_new(
            position,
            wm.bar_height_px,
            style.bg.unwrap_or_else(|| 0x000000.into()),
            wm.font,
            wm.point_size,
            vec![
                Box::new(Workspaces::new(style, highlight, empty_ws)),
                Box::new(CurrentLayout::new(style)),
                Box::new(ActiveWindowName::new(
                    max_active_window_chars,
                    TextStyle {
                        bg: Some(highlight),
                        padding: (6, 4),
                        ..style
                    },
                    true,
                    false,
                )),
                Box::new(custom_text),
                Box::new(right_pad),
                /*Box::new(RootWindowName::new(
                    TextStyle {
                        padding: (4, 2),
                        ..style
                    },
                    false,
                    true,
                ))*/
            ],
        )
        .unwrap()
    }
}
