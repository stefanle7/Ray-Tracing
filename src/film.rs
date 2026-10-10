use glam::Vec3;
use image::{Rgb, RgbImage};

// RGB color with components nominally in [0, 1].
pub type Color = Vec3;

#[derive(Debug, Clone)]
pub struct Film {
    width: u32,         // horizontal resolution, in pixels
    height: u32,        // vertical resolution, in pixels
    pixels: Vec<Color>, // row-major; (0, 0) is the top-left pixel
}

impl Film {
    pub fn new(width: u32, height: u32) -> Self {
        // all pixels black
        Self {
            width,
            height,
            pixels: vec![Color::ZERO; (width * height) as usize],
        }
    }

    // accessors for width and height
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }

    // width / height, fed to Camera::new so the camera and film agree.
    // code from lecture slides
    pub fn aspect(&self) -> f32 {
        self.width as f32 / self.height as f32
    }

    // methods to set/get a pixel
    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        let i = (y * self.width + x) as usize; // row-major: skip y full rows, then x
        self.pixels[i] = color;
    }
    pub fn get_pixel(&self, x: u32, y: u32) -> Color {
        let i = (y * self.width + x) as usize;
        self.pixels[i]
    }

    // **used claude to help generate syntax for this method**
    pub fn save(&self, path: &str) {
        self.to_image().save(path).expect("failed to write PNG");
    }

    // Convert to an 8-bit image buffer (same orientation: row 0 is the top).
    pub fn to_image(&self) -> RgbImage {
        RgbImage::from_fn(self.width, self.height, |x, y| {
            Rgb(to_rgb8(self.get_pixel(x, y)))
        })
    }
}

// Clamp a float color to [0, 1] and quantize each channel to 0..=255.
pub fn to_rgb8(c: Color) -> [u8; 3] {
    let q = |x: f32| (x.clamp(0.0, 1.0) * 255.0).round() as u8;
    [q(c.x), q(c.y), q(c.z)]
}


// **used claude to generate tests for the Film struct**
#[cfg(test)]
mod tests {
    use super::*;
    use glam::vec3;

    const RED: Color = vec3(1.0, 0.0, 0.0);
    const GREEN: Color = vec3(0.0, 1.0, 0.0);
    const BLUE: Color = vec3(0.0, 0.0, 1.0);
    const WHITE: Color = vec3(1.0, 1.0, 1.0);

    #[test]
    fn new_film_is_black_with_correct_size() {
        let film = Film::new(4, 3);
        assert_eq!(film.width(), 4);
        assert_eq!(film.height(), 3);
        assert!((film.aspect() - 4.0 / 3.0).abs() < 1e-6);
        for y in 0..3 {
            for x in 0..4 {
                assert_eq!(film.get_pixel(x, y), Color::ZERO);
            }
        }
    }

    #[test]
    fn set_then_get_pixel() {
        let mut film = Film::new(8, 6);
        film.set_pixel(0, 0, RED);
        film.set_pixel(7, 0, GREEN);
        film.set_pixel(0, 5, BLUE);
        film.set_pixel(7, 5, WHITE);
        film.set_pixel(3, 2, vec3(0.25, 0.5, 0.75));

        assert_eq!(film.get_pixel(0, 0), RED);
        assert_eq!(film.get_pixel(7, 0), GREEN);
        assert_eq!(film.get_pixel(0, 5), BLUE);
        assert_eq!(film.get_pixel(7, 5), WHITE);
        assert_eq!(film.get_pixel(3, 2), vec3(0.25, 0.5, 0.75));
        // neighbours are untouched
        assert_eq!(film.get_pixel(1, 0), Color::ZERO);
        assert_eq!(film.get_pixel(3, 3), Color::ZERO);
    }

    #[test]
    fn storage_is_row_major() {
        let mut film = Film::new(5, 4);
        film.set_pixel(2, 3, RED);
        assert_eq!(film.pixels[3 * 5 + 2], RED);
    }

    #[test]
    fn quantization_clamps_and_rounds() {
        assert_eq!(to_rgb8(vec3(0.0, 0.5, 1.0)), [0, 128, 255]);
        assert_eq!(to_rgb8(vec3(-0.3, 1.7, 0.2)), [0, 255, 51]);
    }

    /// Paint four colored quadrants plus a yellow marker in the top-left
    /// pixel, write a PNG, then read it back and check it pixel by pixel.
    /// The file is left in output/ so you can also open it and look.
    #[test]
    fn save_png_and_read_back() {
        let (w, h) = (64u32, 48u32);
        let mut film = Film::new(w, h);
        for y in 0..h {
            for x in 0..w {
                let c = match (x < w / 2, y < h / 2) {
                    (true, true) => RED,     // top-left
                    (false, true) => GREEN,  // top-right
                    (true, false) => BLUE,   // bottom-left
                    (false, false) => WHITE, // bottom-right
                };
                film.set_pixel(x, y, c);
            }
        }
        film.set_pixel(0, 0, vec3(1.0, 1.0, 0.0)); // orientation marker

        let path = "output/test_film_quadrants.png";
        std::fs::create_dir_all("output").expect("failed to create output folder");
        film.save(path);

        let img = image::open(path).expect("failed to read png").to_rgb8();
        assert_eq!(img.dimensions(), (w, h));
        for y in 0..h {
            for x in 0..w {
                assert_eq!(
                    img.get_pixel(x, y).0,
                    to_rgb8(film.get_pixel(x, y)),
                    "mismatch at ({x}, {y})"
                );
            }
        }
        assert_eq!(img.get_pixel(0, 0).0, [255, 255, 0]); // top-left stays top-left
        assert_eq!(img.get_pixel(w - 1, 0).0, [0, 255, 0]);
        assert_eq!(img.get_pixel(0, h - 1).0, [0, 0, 255]);
        assert_eq!(img.get_pixel(w - 1, h - 1).0, [255, 255, 255]);
    }
}