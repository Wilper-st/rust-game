use crate::piston_window::Transformed;
use crate::text::MeasureText;

use graphics::types::FontSize;
use graphics::{Context, Text};
use opengl_graphics::{GlGraphics, GlyphCache};
trait DrawText {
    fn draw_text(
        &mut self,
        text: &str,
        r: [f64; 4],
        color: [f32; 4],
        size: FontSize,
        halign: TextAlignment,
        valign: TextVerticalAlignment,
        glyphs: &mut GlyphCache,
        c: &Context,
    );
}
