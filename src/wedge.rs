use penrose::{pure::geometry::Rect, x::XConn, Color};
use penrose_ui::{bar::widgets::Widget, Context, Result};

/// A simple 45 degree wedge
#[derive(Debug, Clone, Copy)]
pub struct Wedge {
    only_with_focus: bool,
    start: bool,
    fg: Color,
    bg: Color,
}

impl Wedge {
    fn new(fg: impl Into<Color>, bg: impl Into<Color>, start: bool) -> Self {
        Self {
            only_with_focus: false,
            start,
            fg: fg.into(),
            bg: bg.into(),
        }
    }

    pub fn start(fg: impl Into<Color>, bg: impl Into<Color>) -> Self {
        Self::new(fg, bg, true)
    }

    pub fn end(fg: impl Into<Color>, bg: impl Into<Color>) -> Self {
        Self::new(fg, bg, true)
    }

    /*fn only_with_focus(mut self) -> Self {
        self.only_with_focus = true;
        self
    }*/
}

impl<X: XConn> Widget<X> for Wedge {
    fn draw(&mut self, ctx: &mut Context<'_>, _: usize, f: bool, w: u32, h: u32) -> Result<()> {
        if self.only_with_focus && !f {
            ctx.fill_rect(Rect::new(0, 0, w, h), self.bg)?;
            return Ok(());
        }

        let p = if self.start { 0 } else { h };
        ctx.fill_rect(
            Rect {
                x: p,
                y: p,
                w: h,
                h,
            },
            self.fg,
        )
    }

    fn current_extent(&mut self, _: &mut Context<'_>, h: u32) -> Result<(u32, u32)> {
        Ok((h, h))
    }

    fn is_greedy(&self) -> bool {
        false
    }

    fn require_draw(&self) -> bool {
        false
    }
}
