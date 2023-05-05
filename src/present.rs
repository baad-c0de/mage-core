use crate::PresentInput;

impl<'t> PresentInput<'t> {
    pub fn print_at(&mut self, x: i32, y: i32, text: &[u8], fore_colour: u32, back_colour: u32) {
        let x = if x < 0 { self.width as i32 + x } else { x } as u32;
        let y = if y < 0 { self.height as i32 + y } else { y } as u32;
        let fore_colour = fore_colour | 0xff000000;
        let back_colour = back_colour | 0xff000000;

        let len = (text.len() as u32).min(self.width - x) as usize;
        let i = (y * self.width + x) as usize;

        let fore_image = self.fore_image[i..i + len].iter_mut();
        let back_image = self.back_image[i..i + len].iter_mut();
        let text_image = self.text_image[i..i + len].iter_mut();

        fore_image
            .zip(back_image)
            .zip(text_image)
            .zip(text.iter())
            .for_each(|(((fore, back), text), c)| {
                *fore = fore_colour;
                *back = back_colour;
                *text = *c as u32;
            });
    }
}
