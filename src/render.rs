use crate::camera::Camera;
use crate::film::Film;
use crate::scene::Scene;
use crate::material::shade_pixel;

// Smallest t that counts as a hit lecture 7 slides 4 and 25
// Anything at or behind the start of the ray is ignored.
pub const T_MIN: f32 = 1e-4;

// Pixel (x, y) -> normalized view-plane coords (s, t) (lecture 6 slide 7).
// + 0.5 aims at the center of the pixel. Film row 0 is the top, but t = 0 is
// the bottom of the view plane, so t is flipped.
pub fn pixel_to_st(x: u32, y: u32, width: u32, height: u32) -> (f32, f32) {
    let s = (x as f32 + 0.5) / width as f32;
    let t = 1.0 - (y as f32 + 0.5) / height as f32;
    (s, t)
}

// pseudocode turned into code from lecture 6 slide 4
pub fn render(scene: &Scene, camera: &Camera, film: &mut Film) {
    for y in 0..film.height() {
        for x in 0..film.width() {
            // normalized [0,1] coords
            let (s, t) = pixel_to_st(x, y, film.width(), film.height());
            let ray = camera.ray(s, t); // primary ray

            let color = match scene.hit(&ray, T_MIN, f32::INFINITY) {
                Some(hit) => shade_pixel(&hit, &ray, scene), // Blinn-Phong (lecture 8 slide 23)
                None => scene.background,      // the ray missed everything
            };

            film.set_pixel(x, y, color); // store the result
        }
    }
}