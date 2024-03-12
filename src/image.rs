/// Represents a rectangular collection of chars to render as sprites or
/// screens.
#[derive(Debug)]
pub struct Image {
    /// The width of the image in chars.
    pub width: u32,

    /// The height of the image in chars.
    pub height: u32,

    /// The foreground color of each char in the image.
    pub fore_image: Vec<u32>,

    /// The background color of each char in the image.
    pub back_image: Vec<u32>,

    /// The char to render at each position in the image.
    pub text_image: Vec<u32>,
}

/// A point in 2D space.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

/// A rectangle in 2D space.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    /// Creates a new rectangle with the given position and dimensions.
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Creates a new rectangle from two points.
    ///
    /// The points do not need to be in any particular order. The rectangle will
    /// be the smallest rectangle that contains both points. If the points are
    /// the same, the rectangle will have zero width and height. If the points
    /// are on the same vertical or horizontal line, the rectangle will have
    /// zero width or height respectively.
    ///
    /// # Arguments
    ///
    /// * `p1` - The first point.
    /// * `p2` - The second point.
    ///
    /// # Returns
    ///
    /// A new rectangle that contains both points.
    ///
    pub fn from_points(p1: Point, p2: Point) -> Self {
        let x = p1.x.min(p2.x);
        let y = p1.y.min(p2.y);
        let width = (p1.x - p2.x).unsigned_abs();
        let height = (p1.y - p2.y).unsigned_abs();
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Creates a new rectangle from a point and size.
    ///
    /// The point is the top-left corner of the rectangle.
    ///
    /// # Arguments
    ///
    /// * `p` - The top-left corner of the rectangle.
    /// * `width` - The width of the rectangle.
    /// * `height` - The height of the rectangle.
    ///
    /// # Returns
    ///
    /// A new rectangle with the given top-left corner and dimensions.
    ///
    pub fn from_point_and_size(p: Point, width: u32, height: u32) -> Self {
        Self {
            x: p.x,
            y: p.y,
            width,
            height,
        }
    }

    /// Returns the union of this rectangle and another rectangle.
    ///
    /// The union of two rectangles is the smallest rectangle that contains both
    /// rectangles.
    ///
    /// # Arguments
    ///
    /// * `other` - The other rectangle to union with.
    ///
    /// # Returns
    ///
    /// A new rectangle that contains both rectangles.
    ///
    pub fn union(&self, other: Self) -> Self {
        let x = self.x.min(other.x);
        let y = self.y.min(other.y);
        let width = (self.x + self.width as i32).max(other.x + other.width as i32) - x;
        let height = (self.y + self.height as i32).max(other.y + other.height as i32) - y;
        Self {
            x,
            y,
            width: width as u32,
            height: height as u32,
        }
    }

    /// Returns the intersection of this rectangle and another rectangle.
    ///
    /// The intersection of two rectangles is the largest rectangle that is
    /// contained within both rectangles.
    ///
    /// # Arguments
    ///
    /// * `other` - The other rectangle to intersect with.
    ///
    /// # Returns
    ///
    /// A new rectangle that is contained within both rectangles.
    ///
    pub fn intersect(&self, other: Self) -> Self {
        let x = self.x.max(other.x);
        let y = self.y.max(other.y);
        let width = (self.x + self.width as i32).min(other.x + other.width as i32) - x;
        let height = (self.y + self.height as i32).min(other.y + other.height as i32) - y;
        Self {
            x,
            y,
            width: width as u32,
            height: height as u32,
        }
    }

    /// Creates a new rectangle by clipping this rectangle to the given
    /// dimensions.
    ///
    /// The new rectangle will be the largest rectangle that fits within the
    /// given dimensions and is contained within this rectangle.
    ///
    /// The point returned is the offset of the top-left corner of the new
    /// rectangle within the given original rectangle.  This is useful for
    /// rendering the new rectangle within the original rectangle.
    ///
    /// # Arguments
    ///
    /// * `width` - The width of the rectangle to clip within.
    /// * `height` - The height of the rectangle to clip within.
    ///
    /// # Returns
    ///
    /// A new rectangle that fits within the given dimensions and is contained
    /// within this rectangle.  Also returns the offset of the top-left corner
    /// of the new rectangle within the original rectangle.
    ///
    pub fn clip_within(&self, width: u32, height: u32) -> (Self, Point) {
        (
            self.intersect(Rect::new(0, 0, width, height)),
            Point::new(-(self.x.min(0)), -(self.y.min(0))),
        )
    }
}

/// A single character to render with colour information.
#[derive(Clone, Copy, Debug)]
pub struct Char {
    pub ch: u32,
    pub ink: u32,
    pub paper: u32,
}

impl Char {
    /// Creates a new 8-bit character with the given char, ink and paper
    /// colours. The char is converted to a u32.
    ///
    /// # Arguments
    ///
    /// * `ch` - The char to render.  This is an 8-bit value.
    /// * `ink` - The foreground colour of the char.
    /// * `paper` - The background colour of the char.
    ///
    /// # Returns
    ///
    /// A new character with the given char, ink and paper colours.
    ///
    pub fn new(ch: u8, ink: u32, paper: u32) -> Self {
        Self {
            ch: ch as u32,
            ink,
            paper,
        }
    }

    /// Creates a new 32-bit character with the given char, ink and paper
    /// colours.
    ///
    /// # Arguments
    ///
    /// * `ch` - The char to render.  This is a 32-bit value.
    /// * `ink` - The foreground colour of the char.
    /// * `paper` - The background colour of the char.
    ///
    /// # Returns
    ///
    /// A new character with the given char, ink and paper colours.
    ///
    pub fn new_u32(ch: u32, ink: u32, paper: u32) -> Self {
        Self { ch, ink, paper }
    }

    /// Creates a new character with the given char, ink and paper colours. The
    /// char is converted to a u8.
    ///
    /// # Arguments
    ///
    /// * `ch` - The char to render.  This is a normal Rust char converted to an
    ///   8-bit value.
    /// * `ink` - The foreground colour of the char.
    /// * `paper` - The background colour of the char.
    ///
    pub fn new_char(ch: char, ink: u32, paper: u32) -> Self {
        let char_byte = ch as u8;
        Self::new(char_byte, ink, paper)
    }
}

impl Image {
    /// Creates a new image with the given dimensions.
    ///
    /// # Arguments
    ///
    /// * `width` - The width of the image in chars.
    /// * `height` - The height of the image in chars.
    ///
    /// # Returns
    ///
    /// A new image with the given dimensions.  The image is filled with
    /// character zero.
    ///
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        Self {
            width,
            height,
            fore_image: vec![0; size],
            back_image: vec![0; size],
            text_image: vec![0; size],
        }
    }

    /// Returns the index of the char at the given coordinates.
    ///
    /// # Arguments
    ///
    /// * `p` - The coordinates of the char to get the index of.
    ///
    /// # Returns
    ///
    /// The index of the char at the given coordinates, or `None` if the
    /// coordinates are out of bounds.
    ///
    pub fn point_to_index(&self, p: Point) -> Option<usize> {
        self.coords_to_index(p.x, p.y)
    }

    /// Returns the index of the char at the given coordinates.
    ///
    /// # Arguments
    ///
    /// * `x` - The x coordinate of the char to get the index of.
    /// * `y` - The y coordinate of the char to get the index of.
    ///
    /// # Returns
    ///
    /// The index of the char at the given coordinates, or `None` if the
    /// coordinates are out of bounds.
    ///
    pub fn coords_to_index(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || y < 0 {
            None
        } else {
            let x = x as u32;
            let y = y as u32;
            if x < self.width && y < self.height {
                Some((y * self.width + x) as usize)
            } else {
                None
            }
        }
    }

    /// Clears the image with a given ink and paper colour.
    ///
    /// # Arguments
    ///
    /// * `ink` - The foreground colour to clear the image with.
    /// * `paper` - The background colour to clear the image with.
    ///
    pub fn clear(&mut self, ink: u32, paper: u32) {
        self.fore_image
            .iter_mut()
            .for_each(|fore_image| *fore_image = ink);
        self.back_image
            .iter_mut()
            .for_each(|back_image| *back_image = paper);
        self.text_image
            .iter_mut()
            .for_each(|text_image| *text_image = 0);
    }

    /// Draws a character at the given coordinates.
    ///
    /// # Arguments
    ///
    /// * `p` - The coordinates to draw the character at.
    /// * `ch` - The character to draw.
    ///
    /// # Notes
    ///
    /// If the coordinates are out of bounds, the character is not drawn.
    ///
    pub fn draw_char(&mut self, p: Point, ch: Char) {
        if let Some(index) = self.point_to_index(p) {
            self.fore_image[index] = ch.ink;
            self.back_image[index] = ch.paper;
            self.text_image[index] = ch.ch;
        }
    }

    /// Draws a string at the given coordinates.
    ///
    /// # Arguments
    ///
    /// * `p` - The coordinates to draw the string at.
    /// * `text` - The string to draw.
    /// * `ink` - The foreground colour of the string.
    /// * `paper` - The background colour of the string.
    ///
    /// # Notes
    ///
    /// If the coordinates are out of bounds, the string is clipped.
    ///
    pub fn draw_string(&mut self, p: Point, text: &str, ink: u32, paper: u32) {
        let (text_rect, str_offset) =
            Rect::from_point_and_size(p, text.len() as u32, 1).clip_within(self.width, self.height);
        if str_offset.y == 0 {
            let str_slice =
                &text[str_offset.x as usize..(str_offset.x + text_rect.width as i32) as usize];

            if let Some(i) = self.coords_to_index(text_rect.x, text_rect.y) {
                let w = text_rect.width as usize;
                self.fore_image[i..i + w].iter_mut().for_each(|x| *x = ink);
                self.back_image[i..i + w]
                    .iter_mut()
                    .for_each(|x| *x = paper);
                self.text_image[i..i + w]
                    .iter_mut()
                    .zip(str_slice.bytes())
                    .for_each(|(x, y)| *x = y as u32);
            }
        }
    }

    /// Draws a rectangle at the given coordinates and dimensions using the
    /// given character.
    ///
    /// # Arguments
    ///
    /// * `p` - The coordinates to draw the rectangle at.
    /// * `width` - The width of the rectangle.
    /// * `height` - The height of the rectangle.
    /// * `ch` - The character to draw the rectangle with.
    ///
    /// # Notes
    ///
    /// If the coordinates are out of bounds, the rectangle is clipped.
    ///
    pub fn draw_filled_rect(&mut self, rect: Rect, ch: Char) {
        let (rect, _) = rect.clip_within(self.width, self.height);

        if let Some(mut i) = self.coords_to_index(rect.x, rect.y) {
            let w = rect.width as usize;
            let h = rect.height;
            (0..h).for_each(|_| {
                self.fore_image[i..i + w]
                    .iter_mut()
                    .for_each(|x| *x = ch.ink);
                self.back_image[i..i + w]
                    .iter_mut()
                    .for_each(|x| *x = ch.paper);
                self.text_image[i..i + w]
                    .iter_mut()
                    .for_each(|x| *x = ch.ch);

                i += self.width as usize;
            });
        }
    }

    /// Returns a rectangle representing the bounds of the image.
    ///
    /// # Returns
    ///
    /// A rectangle representing the bounds of the image.
    ///
    /// # Notes
    ///
    /// The rectangle returned is always at the origin.
    ///
    /// The width and height of the rectangle are the same as the width and
    /// height of the image.
    ///
    pub fn rect(&self) -> Rect {
        Rect::from_point_and_size(Point::new(0, 0), self.width, self.height)
    }
}
