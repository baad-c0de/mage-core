use std::num::NonZeroU32;

use bytemuck::cast_slice;
use image::{load_from_memory, EncodableLayout, GenericImageView};

use crate::error::MageError;

pub const MIN_WINDOW_SIZE: (u32, u32) = (20, 20);

/// Defines the size of the window and how it and its cells change when it
/// changes.
pub enum WindowSize {
    /// The cell size is fixed and the number of cells changes to reflect the
    /// window size.  So, when the window is resized, the number of cells
    /// vertically and horizontally changes to reflect the new window size. The
    /// first value is the number of pixels horizontally, and the second value
    /// is the number of pixels vertically.  The number of cells is calculated
    /// from the window size and the cell size.
    ///
    /// When the window is resized, it snaps to the nearest multiple of the cell
    /// size.
    ///
    /// For example, if you choose 320x240 as the window size in pixels, and the
    /// cells are 8x8 pixels, the window's inner size will be 320x240 showing
    /// 40x30 cells.
    FixedCellSize(u32, u32),

    /// The number of cells is fixed and the cell size changes to reflect the
    /// window size.  The first value is the width of the window in cells, and
    /// the second value is the height of the window in cells.
    ///
    /// The initial window size is reduced to the nearest multiple of the cell
    /// size.  This means that the given cell dimensions are shown to fit the
    /// window exactly with an appropriate scale chosen.
    ///
    /// When the window is resized, it snaps to the nearest multiple of the cell
    /// size scaled approriately.
    ///
    /// For example, if you chose 320x240 as the size of the window in cells,
    /// and the cells are 8x8 pixels, the window's inner size will be 2560x1920,
    /// or multiple thereof.
    FixedCellDimensions(u32, u32),

    /// The window size is fixed and cannot be resized.  The first value is the
    /// width of the window in cells, and the second value is the height of the
    /// window in cells.  The third value is the scale of the cells, with 1
    /// meaning no scaling.
    ///
    /// The window is not resizable, and the size of the window is calculated
    /// from the number of cells and scale.
    FixedWindowSize(u32, u32, NonZeroU32),
}

pub struct WindowSizeData {
    /// The size of the window in pixels inside the window's border.
    pub inner_size: (u32, u32),

    /// The size of the window in cells, taking into account the cell size and
    /// scale.
    pub cell_size: (u32, u32),

    /// Window snapping size.  The window size (in pixels) is always a multiple
    /// of this size.
    pub snap_size: (u32, u32),
}

/// Used to store the configuration required to run the Mage game engine.
pub struct Config {
    /// The title of the window.
    pub title: Option<String>,

    /// The size and resizing behaviour of the window.
    pub window_size: WindowSize,

    /// The font to use for rendering.
    pub font: Font,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            title: None,
            window_size: WindowSize::FixedCellSize(800, 600),
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
