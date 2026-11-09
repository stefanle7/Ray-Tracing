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
mod polymesh;

use glam::{Vec3, vec3};
use camera::Camera;
use film::{Color, Film};
use material::Material;
use plane::Plane;
use render::render;
use scene::Scene;
use sphere::Sphere;
use polymesh::{PolyMesh, Triangle};

// sphere colors used in more than one image
const RED: Color = vec3(0.80, 0.16, 0.13);
const GREEN: Color = vec3(0.18, 0.60, 0.28);
const BLUE: Color = vec3(0.16, 0.32, 0.85);
const GOLD: Color = vec3(0.85, 0.62, 0.15);

fn main() {
    // **used claude to generate the following syntax to create
    // the output folder if it doesn't exist**
    std::fs::create_dir_all("output").expect("failed to create output folder");

    //save_image(&image_1_blinn_phong(), "output/1_blinn_phong.png");
    //save_image(&image_2_lambert(), "output/2_lambert.png");
    //save_image(&image_3_mixed(), "output/3_mixed.png");
    //save_image(&image_4_shininess(), "output/4_shininess.png");
    save_image(&image_5_hard_pyramid(), "output/5_hard_pyramid.png");
    save_image(&image_6_hard_vs_smooth(), "output/6_hard_vs_smooth.png");
    save_image(&image_7_obj(), "output/7_obj.png");
}
// render a scene with the same camera every time and save it as a 1024 x 768 PNG
fn save_image(scene: &Scene, path: &str) {
    let mut film = Film::new(1024, 768);

    // where the camera is sat
    let camera = Camera::new(vec3(0.0, 5.0, 10.5), vec3(0.0, 0.6, -0.3), Vec3::Y, 40.0, film.aspect());
    render(scene, &camera, &mut film);
    film.save(path);
    println!("saved {path} succesfully!");
}

// a new scene that has the gray ground plane in it
fn new_scene() -> Scene {
    // **used claude to help me create syntax for grey background**

    // make sure the shadows aren't full black
    let mut scene = Scene::new(vec3(0.12, 0.13, 0.17), Vec3::splat(0.08)); // background, ambient
    let floor = Material::new(vec3(0.75, 0.75, 0.75), Vec3::ZERO, 1.0); // matte light gray
    scene.add_shape(Box::new(Plane::new(Vec3::ZERO, Vec3::Y, floor)));
    scene
}

// (each center's y equals its radius, so the sphere sits on the ground)
fn add_four_spheres(scene: &mut Scene, materials: [Material; 4]) {
    scene.add_shape(Box::new(Sphere::new(vec3(-2.7, 0.8, 0.4), 0.8, materials[0])));
    scene.add_shape(Box::new(Sphere::new(vec3(-0.8, 1.0, -0.8), 1.0, materials[1])));
    scene.add_shape(Box::new(Sphere::new(vec3(1.0, 0.7, 0.9), 0.7, materials[2])));
    scene.add_shape(Box::new(Sphere::new(vec3(2.8, 0.9, -0.5), 0.9, materials[3])));
}

// Image 1: Blinn-Phong (shiny) spheres, warm light + cool light
fn image_1_blinn_phong() -> Scene {
    let mut scene = new_scene();
    let white = Vec3::splat(0.6); // specular color: white highlight
    add_four_spheres(
        // mutates the scene
        &mut scene,
        [
            Material::new(RED, white, 100.0),
            Material::new(GREEN, white, 100.0),
            Material::new(BLUE, white, 100.0),
            Material::new(GOLD, white, 100.0),
        ],
    );
    scene.add_light(vec3(-5.0, 8.0, 6.0), vec3(1.0, 0.85, 0.7) * 0.8); // warm, upper left
    scene.add_light(vec3(6.0, 5.0, 4.0), vec3(0.55, 0.65, 1.0) * 0.5); // cool, right
    scene
}

// Image 2: Lambert (matte) spheres: specular = 0 turns Blinn-Phong into Lambert
fn image_2_lambert() -> Scene {
    let mut scene = new_scene();
    let none = Vec3::ZERO; // no highlight
    add_four_spheres(
        &mut scene,
        [
            Material::new(RED, none, 1.0),
            Material::new(GOLD, none, 1.0),
            Material::new(GREEN, none, 1.0),
            Material::new(BLUE, none, 1.0),
        ],
    );
    scene.add_light(vec3(-2.0, 10.0, 3.0), Vec3::splat(0.75)); // white, overhead
    scene.add_light(vec3(7.0, 6.0, 6.0), Vec3::splat(0.4)); // dimmer white, right side
    scene
}

// Image 3: 2 Lambert + 2 Blinn-Phong spheres, low orange "sunset" light + dim blue sky light
fn image_3_mixed() -> Scene {
    let mut scene = new_scene();
    add_four_spheres(
        &mut scene,
        [
            Material::new(RED, Vec3::ZERO, 1.0),            // Lambert
            Material::new(GOLD, Vec3::splat(0.8), 300.0),   // Blinn-Phong, tight highlight
            Material::new(GREEN, Vec3::ZERO, 1.0),          // Lambert
            Material::new(BLUE, Vec3::splat(0.8), 30.0),    // Blinn-Phong, broad highlight
        ],
    );
    scene.add_light(vec3(-12.0, 3.0, 3.0), vec3(1.0, 0.6, 0.3) * 1.1); // low sun: long shadows
    scene.add_light(vec3(5.0, 9.0, 6.0), vec3(0.4, 0.55, 0.9) * 0.45); // dim blue sky
    scene
}

// Image 4: 4 spheres of the same color, shininess p = 10, 100, 1000, 10000
// (the values from lecture 8 slide 21)
fn image_4_shininess() -> Scene {
    let mut scene = new_scene();
    let shininess = [10.0, 100.0, 1000.0, 10000.0];

    // four iterations of p 
    for i in 0..4 {
        let x = -3.3 + 2.2 * i as f32; // spread them out left to right
        let material = Material::new(BLUE, Vec3::splat(0.6), shininess[i]);
        scene.add_shape(Box::new(Sphere::new(vec3(x, 0.9, 0.0), 0.9, material)));
    }
    scene.add_light(vec3(-4.0, 7.0, 8.0), Vec3::splat(0.8)); // main light
    scene.add_light(vec3(6.0, 4.0, 5.0), Vec3::splat(0.35)); // fill light
    scene
}







// Ray Tracer Stage 3: Hardcoded a simple mesh 
// (a pyramid for hard edges and several connected tirangles for smooth edges would be adequate for a first test)
fn pyramid_vertices(x: f32) -> Vec<Vec3> {
    vec![
        vec3(x - 1.0, 0.0, -1.0), 
        vec3(x + 1.0, 0.0, -1.0),  
        vec3(x + 1.0, 0.0, 1.0),   
        vec3(x - 1.0, 0.0, 1.0),  
        vec3(x + 0.0, 2.0, 0.0),   
    ]
}

fn hard_pyramid(x: f32, material: Material) -> PolyMesh {
    PolyMesh::new(
        pyramid_vertices(x),
        vec![
            vec3(0.0, 1.0, 2.0).normalize(),  
            vec3(2.0, 1.0, 0.0).normalize(),  
            vec3(0.0, 1.0, -2.0).normalize(), 
            vec3(-2.0, 1.0, 0.0).normalize(), 
        ],
        vec![
            Triangle { v: [3, 2, 4], n: [0, 0, 0] }, // front
            Triangle { v: [2, 1, 4], n: [1, 1, 1] }, // right
            Triangle { v: [1, 0, 4], n: [2, 2, 2] }, // back
            Triangle { v: [0, 3, 4], n: [3, 3, 3] }, // left
        ],
        material,
    )
}

// Image 5: part 3 mesh test, a pyramid with hard edges
fn image_5_hard_pyramid() -> Scene {
    let mut scene = new_scene();
    let material = Material::new(RED, Vec3::splat(0.5), 50.0);
    scene.add_shape(Box::new(hard_pyramid(0.0, material)));
    scene.add_light(vec3(-4.0, 10.0, 7.0), Vec3::splat(0.8));
    scene.add_light(vec3(5.0, 9.0, 6.0), Vec3::splat(0.35));
    scene
}

fn smooth_pyramid(x:f32, material: Material) -> PolyMesh {
    PolyMesh::new(
        pyramid_vertices(x),
        vec![
            vec3(-2.0, 2.0, -2.0).normalize(), 
            vec3(2.0, 2.0, -2.0).normalize(),  
            vec3(2.0, 2.0, 2.0).normalize(),   
            vec3(-2.0, 2.0, 2.0).normalize(),  
            vec3(0.0, 1.0, 0.0),               
        ],
        vec![
            Triangle { v: [3, 2, 4], n: [0, 1, 4] }, 
            Triangle { v: [2, 1, 4], n: [1, 2, 4] }, 
            Triangle { v: [1, 0, 4], n: [2, 3, 4] }, 
            Triangle { v: [0, 3, 4], n: [3, 0, 4] }, 
        ],
        material,
    )
}

// Image 6: part 3 mesh test, hard edges (left) vs smooth edges (right)
fn image_6_hard_vs_smooth() -> Scene {
    let mut scene = new_scene();
    let material = Material::new(RED, Vec3::splat(0.5), 50.0);
    scene.add_shape(Box::new(hard_pyramid(-1.6, material)));
    scene.add_shape(Box::new(smooth_pyramid(1.6, material)));
    scene.add_light(vec3(-4.0, 10.0, 7.0), Vec3::splat(0.8));
    scene.add_light(vec3(5.0, 9.0, 6.0), Vec3::splat(0.35));
    scene
}

// Image 7: part 4, a model read from an .obj file
fn image_7_obj() -> Scene {
    let mut scene = new_scene();
    let mesh = PolyMesh::load_obj("models/kenny.obj", Material::new(BLUE, Vec3::splat(0.5), 50.0));
    scene.add_shape(Box::new(mesh));
    scene.add_light(vec3(-4.0, 10.0, 7.0), Vec3::splat(0.8));
    scene.add_light(vec3(5.0, 9.0, 6.0), Vec3::splat(0.35));
    scene
}