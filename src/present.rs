use crate::{
    image::{Image, Rect},
    PresentInput,
};

impl<'t> PresentInput<'t> {
    pub fn rect(&self) -> Rect {
        Rect {
            x: 0,
            y: 0,
            width: self.width,
            height: self.height,
        }
    }

    pub fn new_image(&self) -> Image {
        Image::new(self.width, self.height)
    }

    /// Blit the an area of the source image to the screen.
    ///
    /// The source rectangle is clipped to the source image.
    ///
    /// # Arguments
    ///
    /// * `dst_rect` - Where to blit the source image to on the screen.
    /// * `src_rect` - The area of the source image to blit.
    /// * `src_image` - The source image to blit from.
    /// * `paper` - The paper colour to use outside the source image.  This will
    ///   also be used as the ink colour.
    ///
    pub fn blit(&mut self, mut dst_rect: Rect, src_rect: Rect, src_image: &Image, paper: u32) {
        assert_eq!(dst_rect.width, src_rect.width);
        assert_eq!(dst_rect.height, src_rect.height);
        assert!(dst_rect.x >= 0 && dst_rect.y >= 0);
        assert!(dst_rect.x + dst_rect.width as i32 <= self.width as i32);
        assert!(dst_rect.y + dst_rect.height as i32 <= self.height as i32);

        // Clip the source rectangle to the source image and adjust the
        // destination rectangle accordingly.
        let (src_rect, src_offset) = src_rect.clip_within(src_image.width, src_image.height);
        dst_rect.width = src_rect.width;
        dst_rect.height = src_rect.height;

        if src_rect.width == 0 || src_rect.height == 0 {
            // Nothing to blit, so clear it
            self.clear(dst_rect, paper);
        } else {
            // Clear the top-left corner of the destination rectangle according
            // to any offset in the source rectangle.
            self.clear(
                Rect {
                    x: dst_rect.x,
                    y: dst_rect.y,
                    width: src_offset.x as u32,
                    height: src_offset.y as u32,
                },
                paper,
            );

            // Clear the top-right corner of the destination rectangle according
            // to any offset in the source rectangle.
            self.clear(
                Rect {
                    x: dst_rect.x + src_offset.x,
                    y: dst_rect.y,
                    width: dst_rect.width - src_offset.x as u32,
                    height: src_offset.y as u32,
                },
                paper,
            );

            // Clear the bottom-left corner of the destination rectangle according
            // to any offset in the source rectangle.
            self.clear(
                Rect {
                    x: dst_rect.x,
                    y: dst_rect.y + src_offset.y,
                    width: src_offset.x as u32,
                    height: dst_rect.height - src_offset.y as u32,
                },
                paper,
            );

            // Blit the image to the screen.
            self.blit_internal(
                Rect {
                    x: dst_rect.x + src_offset.x,
                    y: dst_rect.y + src_offset.y,
                    width: dst_rect.width - src_offset.x as u32,
                    height: dst_rect.height - src_offset.y as u32,
                },
                src_rect,
                src_image,
            );
        }
    }

    pub fn clear(&mut self, rect: Rect, paper: u32) {
        assert!(rect.x >= 0 && rect.y >= 0);
        assert!(rect.x + rect.width as i32 <= self.width as i32);
        assert!(rect.y + rect.height as i32 <= self.height as i32);

        let mut i = rect.y as usize * self.width as usize + rect.x as usize;
        for _ in 0..rect.height {
            self.fore_image[i..i + rect.width as usize].fill(paper);
            self.back_image[i..i + rect.width as usize].fill(paper);
            self.text_image[i..i + rect.width as usize].fill(0);
            i += self.width as usize;
        }
    }

    pub fn blit_internal(&mut self, dst_rect: Rect, src_rect: Rect, src_image: &Image) {
        assert_eq!(dst_rect.width, src_rect.width);
        assert_eq!(dst_rect.height, src_rect.height);
        assert!(dst_rect.x >= 0 && dst_rect.y >= 0);
        assert!(dst_rect.x + dst_rect.width as i32 <= self.width as i32);
        assert!(dst_rect.y + dst_rect.height as i32 <= self.height as i32);

        let mut dst_i = dst_rect.y as usize * self.width as usize + dst_rect.x as usize;
        let mut src_i = src_rect.y as usize * src_image.width as usize + src_rect.x as usize;
        for _ in 0..dst_rect.height {
            self.fore_image[dst_i..dst_i + dst_rect.width as usize]
                .copy_from_slice(&src_image.fore_image[src_i..src_i + dst_rect.width as usize]);
            self.back_image[dst_i..dst_i + dst_rect.width as usize]
                .copy_from_slice(&src_image.back_image[src_i..src_i + dst_rect.width as usize]);
            self.text_image[dst_i..dst_i + dst_rect.width as usize]
                .copy_from_slice(&src_image.text_image[src_i..src_i + dst_rect.width as usize]);
            dst_i += self.width as usize;
            src_i += src_image.width as usize;
        }
    }
}
