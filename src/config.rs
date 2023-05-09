use bytemuck::cast_slice;
use image::{load_from_memory, EncodableLayout, GenericImageView};

use crate::error::MageError;

pub const MIN_WINDOW_SIZE: (u32, u32) = (20, 20);

/// Used to store the configuration required to run the Mage game engine.
pub struct Config {
    /// The title of the window.
    pub title: Option<String>,

    /// The size of the window in characters.
    ///
    /// The first value is the width in pixels, and the second value is the
    /// height in pixels.
    ///
    /// The dimensions can not go below the number of pixels required to display
    /// 20 characters in each direction.
    pub inner_size: (u32, u32),

    /// The font to use for rendering.
    pub font: Font,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            title: None,
            inner_size: (800, 600),
            font: Font::Default,
        }
    }
}

/// The [`FontData`] struct is used to store the data required to load a custom
/// font.
///
/// [`FontData`]: struct.FontData.html
///
pub enum Font {
    /// The default built-in font.
    Default,

    /// A custom font determined by the application.
    Custom(FontData),
}

/// The [`FontData`] struct is used to store the data required to load a custom
/// font.
///
/// [`FontData`]: struct.FontData.html
///
pub struct FontData {
    /// The RGBA data of the font.
    pub data: Vec<u32>,

    /// The width of each character in pixels.
    pub char_width: u32,

    /// The height of each character in pixels.
    pub char_height: u32,
}

pub fn load_font_image(data: &[u8]) -> Result<FontData, MageError> {
    let font_image = load_from_memory(data)?;
    let dimensions = font_image.dimensions();
    let font_rgba = font_image.to_rgba8();
    let font_data = font_rgba.as_bytes();
    let data_u32: &[u32] = cast_slice(font_data);
    let char_width = dimensions.0 / 16;
    let char_height = dimensions.1 / 16;
    if char_width == 0
        || char_height == 0
        || char_width * 16 != dimensions.0
        || char_height * 16 != dimensions.1
    {
        return Err(MageError::InvalidFontImage);
    }

    Ok(FontData {
        data: data_u32.to_vec(),
        char_width,
        char_height,
    })
}
