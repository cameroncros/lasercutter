use std::io::Cursor;

use base64::Engine;
use image::{ImageFormat, Rgb, RgbImage};

use crate::{
    gcode_emulator::renderer::{RenderSettings, Renderer},
    types::coord::Coord,
};

#[derive(Clone)]
pub struct PixelRenderer {
    image: RgbImage,
    pw: u32,
    ph: u32,
}

impl PixelRenderer {
    pub fn new() -> Self {
        PixelRenderer {
            image: RgbImage::from_pixel(700, 700, Rgb([255, 255, 255])),
            pw: 1,
            ph: 1,
        }
    }
    fn set_pixel(&mut self, x: u32, y: u32, pixel: Rgb<u8>) {
        for i in 0..self.pw {
            for j in 0..self.ph {
                self.image
                    .put_pixel(x * self.pw + i, y * self.ph + j, pixel);
            }
        }
    }
}

impl Renderer for PixelRenderer {
    fn draw_line(
        &mut self,
        start: Coord,
        end: Coord,
        render_settings: RenderSettings,
    ) -> anyhow::Result<()> {
        // Perform the line drawing on the logical pixel grid (before scaling by pw/ph)
        let (x1, y1, x2, y2) = (start.0 as i32, start.1 as i32, end.0 as i32, end.1 as i32);

        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();
        let steps = dx.max(dy) as usize;

        let (r, g, b) = match render_settings.color.as_str() {
            "green" => (0, 255, 0),
            "red" => (255, 0, 0),
            _ => unreachable!(),
        };
        let color = Rgb([r, g, b]);

        // Precompute bounds in logical pixel units
        let max_x = (self.image.width() / self.pw) as i32;
        let max_y = (self.image.height() / self.ph) as i32;

        if steps == 0 {
            if x1 >= 0 && x1 < max_x && y1 >= 0 && y1 < max_y {
                self.set_pixel(x1 as u32, y1 as u32, color);
            }
            return Ok(());
        }

        let x_inc = (x2 - x1) as f32 / steps as f32;
        let y_inc = (y2 - y1) as f32 / steps as f32;

        let mut xf = x1 as f32;
        let mut yf = y1 as f32;

        for i in 0..=steps {
            let _t = i as f32 / steps as f32;

            let xi = xf.round() as i32;
            let yi = yf.round() as i32;

            if xi >= 0 && xi < max_x && yi >= 0 && yi < max_y {
                self.set_pixel(xi as u32, yi as u32, Rgb([r, g, b]));
            }

            xf += x_inc;
            yf += y_inc;
        }
        Ok(())
    }

    fn to_file(&mut self, file: &str) -> anyhow::Result<()> {
        Ok(self.image.save(file)?)
    }

    fn to_img_url(&mut self) -> anyhow::Result<String> {
        let mut buffer = Cursor::new(Vec::new());
        self.image.write_to(&mut buffer, ImageFormat::Png)?;

        let svg_b64 = base64::engine::general_purpose::STANDARD.encode(buffer.into_inner());
        Ok(format!("data:image/png;base64,{}", svg_b64))
    }
}
