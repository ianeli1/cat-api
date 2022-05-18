use cairo::{Context, FontSlant, FontWeight, ImageSurface};
use derive_more::From;
use std::io::Cursor;

pub struct CatImage {
    image: ImageSurface,
    ctx: Context,
}

#[derive(From, Debug)]
pub enum CairoError {
    IoError(cairo::IoError),
    Error(cairo::Error),
}

impl CatImage {}

impl CatImage {
    pub fn draw(&self) {
        self.ctx
            .select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
        self.ctx.set_font_size(10.0);

        self.ctx.move_to(0.04, 0.53);
        self.ctx.show_text("Hello").expect("Oops");
    }

    pub fn write_to(&self, out: &mut Vec<u8>) -> Result<(), CairoError> {
        self.ctx.paint()?;
        self.image.write_to_png(out)?;
        Ok(())
    }

    pub fn from_vec_cursor(cursor: &mut Cursor<Vec<u8>>) -> Result<CatImage, CairoError> {
        let surface = ImageSurface::create_from_png(cursor)?;

        Ok(CatImage {
            ctx: Context::new(&surface)?,
            image: surface,
        })
    }
}
