use glam::Vec3;
use crate::material::Material;
// I'm using my previous hit shape 
use crate::shape::{Hit, Shape};


// part 1 
// triangle struct function
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle {
    pub v: [usize; 3], // indices of vertices 
    pub n: [usize; 3], // indices of normals
}

// polymesh struct function
#[derive(Debug, Clone)]
pub struct PolyMesh {
    pub vertices: Vec<Vec3>, // list of vertices
    pub normals: Vec<Vec3>,  // list of normals
    pub triangles: Vec<Triangle>, // list of triangles  
    pub material: Material, // material of the polymesh    
}

// polymesh function
impl PolyMesh {
    // constructor for polymesh
    pub fn new(vertices: Vec<Vec3>, normals: Vec<Vec3>, triangles: Vec<Triangle>, material: Material) -> Self {
        Self {
            vertices,
            normals,
            triangles,
            material,
        }
    }
    // ability to iterate through the array of triangles to fetch vertices (and normals) for each triangle
    
    // the corner points of triangle i (index lookup, lecture 12 slide 5)
    // I don't need any for loops since passing in i loops over all triangles
    pub fn corners(&self, i: usize) -> [Vec3; 3] {
        let tri = &self.triangles[i];
        [self.vertices[tri.v[0]], self.vertices[tri.v[1]], self.vertices[tri.v[2]]]
    }

    // the 3 corner normals of triangle i
    // same as above, don't need any for loops since passing in i loops over all triangles
    pub fn corner_normals(&self, i: usize) -> [Vec3; 3] {
        let tri = &self.triangles[i];
        [self.normals[tri.n[0]], self.normals[tri.n[1]], self.normals[tri.n[2]]]
    }

    // part 4: simplified .obj reader (only v, vn and triangle f lines)
    // **used claude to help with the Rust syntax for reading the file and splitting the lines**
    pub fn load_obj(path: &str, material: Material) -> PolyMesh {
        let text = std::fs::read_to_string(path).unwrap();
        let mut vertices = Vec::new();
        let mut normals = Vec::new();
        let mut triangles = Vec::new();

        for line in text.lines() {
            let w: Vec<&str> = line.split_whitespace().collect(); // the words on this line

            if line.starts_with("v ") {
                vertices.push(Vec3::new(w[1].parse().unwrap(), w[2].parse().unwrap(), w[3].parse().unwrap()));
            } else if line.starts_with("vn ") {
                normals.push(Vec3::new(w[1].parse().unwrap(), w[2].parse().unwrap(), w[3].parse().unwrap()).normalize());
            } else if line.starts_with("f ") {
                // each corner is "v//vn" or "v/vt/vn": first number = vertex, last number = normal
                let mut v = [0; 3];
                let mut n = [0; 3];
                for k in 0..3 {
                    let ids: Vec<&str> = w[k + 1].split('/').collect();
                    v[k] = ids[0].parse::<usize>().unwrap() - 1; // .obj counts from 1, Rust from 0
                    n[k] = ids[2].parse::<usize>().unwrap() - 1;
                }
                triangles.push(Triangle { v, n });
            }
        }
        PolyMesh::new(vertices, normals, triangles, material)
    }
}

// part 2 polymesh hit

// taken from lecture 12 slide 10, ray-triangle intersection slides
// ray e + t d against triangle (a, b, c) -> (t, beta, gamma) of the hit (lecture 12 slide 10)
fn hit_triangle(a: Vec3, b: Vec3, c: Vec3, e: Vec3, d: Vec3, t_min: f32, t_max: f32)
        -> Option<(f32, f32, f32)> {
    let e1 = b - a;                         // the two edge vectors
    let e2 = c - a;
    let p = d.cross(e2);
    let det = e1.dot(p);                    // = the determinant of the 3x3 system
    if det.abs() < 1e-8 { return None; }    // ray parallel to the plane
    let inv = 1.0 / det;
    let s = e - a;
    let beta = s.dot(p) * inv;              // Cramer's rule, one unknown at a time ...
    if beta < 0.0 || beta > 1.0 { return None; }
    let q = s.cross(e1);
    let gamma = d.dot(q) * inv;             // ... bailing out as soon as one fails
    if gamma < 0.0 || beta + gamma > 1.0 { return None; }
    let t = e2.dot(q) * inv;
    if t < t_min || t > t_max { return None; }
    Some((t, beta, gamma))
}

// interpolate the normal at the hit point
// shading normal (lecture 12 slide 16), adapted to my PolyMesh and Hit
fn shading_normal(mesh: &PolyMesh, tri: usize, beta: f32, gamma: f32) -> Vec3 {
    let [i, j, k] = mesh.triangles[tri].n;  // my triangles keep normal indices in .n
    let alpha = 1.0 - beta - gamma;
    let n = alpha * mesh.normals[i as usize]
          + beta  * mesh.normals[j as usize]
          + gamma * mesh.normals[k as usize];
    n.normalize()                           // the blend is < 1 long
}

// implemeting shape for polymesh now instead of sphere (previous stage)
// lecture 12 slide 5 
impl Shape for PolyMesh {
    fn hit(&self, e: Vec3, d: Vec3, t_min: f32, t_max: f32) -> Option<Hit> {
        let mut best: Option<Hit> = None;       // best hit so far 
        let mut closest = t_max;                // closest hit so far 
        for t in 0..self.triangles.len() {      // iterating throuhg the triangles
            let [a,b,c] = self.corners(t);      // get the corners of triangle t
            if let Some((th, beta, gamma)) = hit_triangle(a, b, c, e, d, t_min, closest) {
                closest = th;
                let n = shading_normal(self, t, beta, gamma); // getting the shading normal for triangle t
                best = Some(Hit {t: th, p: e + th * d, n, material: self.material});
            }
        }  
        best
    }    
}