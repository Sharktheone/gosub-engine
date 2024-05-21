use vello::glyph::Glyph;
use vello::peniko::{Blob, Font};

use gosub_typeface::{BACKUP_FONT, DEFAULT_LH, FONT_RENDERER_CACHE};

use crate::VelloRenderer;

use gosub_render_backend::{PreRenderText as TPreRenderText, Size, Text as TText, FP};

pub struct Text {
    glyphs: Vec<Glyph>,
}

pub struct PreRenderText {
    text: String,
    fs: FP,
    font: Vec<Font>,
    line_height: FP,
    size: Option<Size>,
    glyphs: Option<Vec<Glyph>>,
}

impl TText<VelloRenderer> for Text {
    fn new(pre: &PreRenderText) -> Self {
        todo!()
    }
}

fn get_fonts_from_family(font_families: Option<Vec<String>>) -> Vec<Font> {
    let mut fonts = Vec::with_capacity(font_families.as_ref().map(|f| f.len()).unwrap_or(1));

    if let Ok(mut cache) = FONT_RENDERER_CACHE.lock() {
        if let Some(ff) = font_families {
            let font = cache.query_all_shared(ff);
            for (i, f) in font.into_iter().enumerate() {
                fonts.push(Font::new(Blob::new(f), i as u32));
            }
        }
    } else {
        fonts.push(Font::new(Blob::new(BACKUP_FONT.data), 0));
    }

    fonts
}

impl TPreRenderText<VelloRenderer> for PreRenderText {
    fn new(text: String, font: Option<Vec<String>>, size: FP) -> Self {
        let font = get_fonts_from_family(font);

        PreRenderText {
            text,
            font,
            line_height: DEFAULT_LH,
            size: None,
            fs: size,
            glyphs: None,
        }
    }

    fn with_lh(text: String, font: Option<Vec<String>>, size: FP, line_height: FP) -> Self {
        let font = get_fonts_from_family(font);

        PreRenderText {
            text,
            font,
            line_height,
            size: None,
            fs: size,
            glyphs: None,
        }
    }

    fn prerender(&mut self, backend: &VelloRenderer) -> Size {
        todo!()
    }

    fn value(&self) -> &str {
        self.text.as_ref()
    }

    fn font(&self) -> Option<&[String]> {
        self.font.as_deref()
    }

    fn fs(&self) -> FP {
        self.fs
    }
}
