use gosub_render_backend::layout::TextLayout as TLayout;
use gosub_render_backend::Size;
use gosub_typeface::font::Font as TFont;
use gosub_typeface::font::Glyph;
use parley::Font as PFont;

#[derive(Debug, Clone)]
pub struct Font(pub PFont);

impl TFont for Font {
    fn to_bytes(&self) -> &[u8] {
        self.0.data.data()
    }
}

impl From<Font> for PFont {
    fn from(font: Font) -> Self {
        font.0
    }
}

#[derive(Debug)]
pub struct TextLayout {
    pub glyphs: Vec<Glyph>,
    pub font: Font,
    pub font_size: f32,
    pub size: Size,
    pub coords: Vec<i16>,
}

impl TLayout for TextLayout {
    type Font = Font;

    fn dbg_layout(&self) -> String {
        format!("TextLayout: {:?}", self)
    }

    fn size(&self) -> Size {
        self.size
    }

    fn glyphs(&self) -> &[Glyph] {
        &self.glyphs
    }

    fn font(&self) -> &Self::Font {
        &self.font
    }

    fn font_size(&self) -> f32 {
        self.font_size
    }

    fn coords(&self) -> &[i16] {
        &self.coords
    }
}
