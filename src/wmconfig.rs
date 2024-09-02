use std::u8;

use penrose_ui::core::TextStyle;

use crate::colordefs::WmColors;
pub struct WmConfig<'a> {
    pub max_main: u32,
    pub ratio: f32,
    pub ratio_step: f32,
    pub outer_px: u32,
    pub inner_px: u32,
    pub bar_height_px: u32,
    pub point_size: u8,
    pub font: &'a str,
    pub text_style: TextStyle,
    pub battery: i32,
    pub notify_count: u8,
    pub flag: i32,
}

impl<'a> Default for WmConfig<'a> {
    fn default() -> Self {
        let style = TextStyle {
            fg: WmColors::white().into(),
            bg: Some(WmColors::black().into()),
            padding: (2, 2),
        };
        Self {
            max_main: 1,
            ratio: 0.6,
            ratio_step: 0.1,
            outer_px: 5,
            inner_px: 5,
            bar_height_px: 18,
            point_size: 8,
            font: "ProFontIIx Nerd Font",
            text_style: style,
            battery: 0,
            notify_count: 0,
            flag: 0
        }
    }
}
