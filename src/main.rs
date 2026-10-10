// tell Rust about every file in src/
mod camera;
mod film;
mod material;
mod plane;
mod ray;
mod render;
mod scene;
mod shape;
mod sphere;
mod transform;

use glam::{Vec3, vec3};
use camera::Camera;
use film::Film;
use material::Material;
use plane::Plane;
use render::render;
use scene::Scene;
use sphere::Sphere;

fn main() {
    // background color, ambient light (Ia)
    let mut scene = Scene::new(vec3(0.12, 0.13, 0.17), Vec3::splat(0.08));

    // ground plane: a point on it, its normal (straight up), a gray matte material
    let gray = Material::new(vec3(0.6, 0.6, 0.6), Vec3::ZERO, 1.0);
    scene.add_shape(Box::new(Plane::new(Vec3::ZERO, Vec3::Y, gray)));

    // Material::new(diffuse color, specular color, shininess p)
    let red = Material::new(vec3(0.80, 0.16, 0.13), Vec3::splat(0.6), 100.0);
    let gold = Material::new(vec3(0.85, 0.62, 0.15), Vec3::splat(0.6), 100.0);
    let green = Material::new(vec3(0.18, 0.60, 0.28), Vec3::splat(0.6), 100.0);
    let blue = Material::new(vec3(0.16, 0.32, 0.85), Vec3::splat(0.6), 100.0);

    // 4 spheres sitting on the ground: each center's y equals its radius
    scene.add_shape(Box::new(Sphere::new(vec3(-2.7, 0.8, 0.4), 0.8, red)));
    scene.add_shape(Box::new(Sphere::new(vec3(-0.8, 1.0, -0.8), 1.0, gold)));
    scene.add_shape(Box::new(Sphere::new(vec3(1.0, 0.7, 0.9), 0.7, green)));
    scene.add_shape(Box::new(Sphere::new(vec3(2.8, 0.9, -0.5), 0.9, blue)));

    // two point lights: position, intensity (brightness and color)
    scene.add_light(vec3(-5.0, 8.0, 6.0), Vec3::splat(0.8));
    scene.add_light(vec3(6.0, 5.0, 4.0), Vec3::splat(0.4));

    // **used claude to generate the following syntax to create
    // the output folder if it doesn't exist**
    std::fs::create_dir_all("output").expect("failed to create output folder");

    // one camera, looking down at the spheres
    let mut film = Film::new(1024, 768);
    let camera = Camera::new(vec3(0.0, 5.0, 10.5), vec3(0.0, 0.6, -0.3), Vec3::Y, 40.0, film.aspect());
    render(&scene, &camera, &mut film);
    film.save("output/test.png");
    println!("saved output/test.png");
}