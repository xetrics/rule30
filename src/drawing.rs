use crate::color::Color;
use font_kit::family_name::FamilyName;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;
use fontdue::Font;
use fontdue::layout::{CoordinateSystem, Layout, LayoutSettings, TextStyle};

pub fn load_font() -> Font {
    let source = SystemSource::new();
    let font_handle = source
        .select_best_match(
            &[FamilyName::Title("Arial".to_string())],
            &Properties::new(),
        )
        .expect("Font not found");

    let fk_font = font_handle.load().expect("Failed to load font");
    let font_data = fk_font
        .copy_font_data()
        .expect("Could not extract raw font data bytes from font");

    let fontdue_settings = fontdue::FontSettings::default();
    let fd_font = fontdue::Font::from_bytes(&font_data[..], fontdue_settings)
        .expect("Fontdue failed to parse the font bytes");

    fd_font
}

pub fn draw_text(
    screen: &mut [u8],
    screen_width: usize,
    screen_height: usize,
    font: &Font,
    text: &str,
    size: f32,
    start_x: usize,
    start_y: usize,
    color: Color,
) {
    let mut layout = Layout::new(CoordinateSystem::PositiveYDown);

    layout.reset(&LayoutSettings {
        x: start_x as f32,
        y: start_y as f32,
        ..LayoutSettings::default()
    });
    layout.append(&[font], &TextStyle::new(text, size, 0));

    for glyph in layout.glyphs() {
        if glyph.width == 0 || glyph.height == 0 {
            continue; // spaces, etc.
        }

        let (metrics, coverage) = font.rasterize_config(glyph.key);
        let gx = glyph.x as i32;
        let gy = glyph.y as i32;

        for row in 0..metrics.height {
            for col in 0..metrics.width {
                let alpha = coverage[row * metrics.width + col] as u32;
                if alpha == 0 {
                    continue;
                }
                let px = gx + col as i32;
                let py = gy + row as i32;
                if px < 0 || py < 0 || px >= screen_width as i32 || py >= screen_height as i32 {
                    continue; // clip to the frame
                }
                let i = ((py as u32 * screen_width as u32 + px as u32) * 4) as usize;
                // Blend text color over whatever is already there
                for c in 0..3 {
                    let dst = screen[i + c] as u32;
                    screen[i + c] = ((color.0[c] as u32 * alpha + dst * (255 - alpha)) / 255) as u8;
                }
                screen[i + 3] = 255;
            }
        }
    }
}

pub fn text_width(font: &Font, text: &str, size: f32) -> usize {
    let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
    layout.append(&[font], &TextStyle::new(text, size, 0));
    layout
        .glyphs()
        .last()
        .map_or(0, |g| (g.x + g.width as f32).ceil() as usize)
}

pub fn draw_square(
    screen: &mut [u8],
    screen_width: usize,
    screen_height: usize,
    start_x: usize,
    start_y: usize,
    size: usize,
    color: Color,
) {
    for dy in 0..size {
        let y = start_y + dy;
        if y >= screen_height {
            break;
        }

        for dx in 0..size {
            let x = start_x + dx;
            if x >= screen_width {
                break;
            }

            let idx = (y * screen_width + x) * 4;
            screen[idx..idx + 4].copy_from_slice(&color.0);
        }
    }
}
