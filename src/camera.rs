use glam::Vec3;

use crate::ray::Ray;

#[derive(Debug, Clone, Copy)]

// taken from the pseudocode in the lecture slides
pub struct Camera {
    origin: Vec3, // eye position in world space
    u: Vec3,      // unit "right" vector
    v: Vec3,      // unit "up" vector
    w: Vec3,      // unit "backward" vector; the camera looks down -w

    lower_left: Vec3, // lower-left corner of the view plane (world space)
    horizontal: Vec3, // full width of the view plane along u
    vertical: Vec3,   // full height of the view plane along v
}

impl Camera {

    // taken from lecture slides again
    pub fn new(eye: Vec3, target: Vec3, up: Vec3, vfov_deg: f32, aspect: f32) -> Self {
        
        // sanity checks for the camera parameters **asked AI to syntheize**
        assert!(eye != target, "Camera::new: eye and target must differ");
        assert!(
            vfov_deg > 0.0 && vfov_deg < 180.0,
            "Camera::new: vfov must be in (0, 180) degrees"
        );
        assert!(aspect > 0.0, "Camera::new: aspect must be positive");

        // taken from lecture slides
        let theta = vfov_deg.to_radians();
        let h = (theta * 0.5).tan();
        let viewport_h = 2.0 * h; // view-plane height at d = 1
        let viewport_w = aspect * viewport_h; // ...and its width

        // orthonormal basis (right-handed; the camera looks down -w)
        let w = (eye - target).normalize(); // backward
        let u = up.cross(w).normalize(); // right
        let v = w.cross(u); // true up (already unit)

        let horizontal = viewport_w * u;
        let vertical = viewport_h * v;
        let lower_left = eye - horizontal * 0.5 - vertical * 0.5 - w;

        // construct the camera instance
        Self {
            origin: eye,
            u,
            v,
            w,
            lower_left,
            horizontal,
            vertical,
        }
    }

    /// Primary ray through normalized view-plane coordinate (s, t) in [0, 1].
    pub fn ray(&self, s: f32, t: f32) -> Ray {
        // walks through the view plane, s moves right and t moves up
        let p = self.lower_left + s * self.horizontal + t * self.vertical;

        // perspective: every ray leaves the eye (direction is not normalized)
        Ray::new(self.origin, p - self.origin)
    }

    /// Camera basis (u, v, w) = (right, up, backward).
    pub fn basis(&self) -> (Vec3, Vec3, Vec3) {
        (self.u, self.v, self.w)
    }
}


//**used claude to generate tests to make sure the Ray is correctly constructed from the camera**
#[cfg(test)]
mod tests {
    use super::*;
    use glam::vec3;

    const EPS: f32 = 1e-5;

    fn assert_vec_eq(actual: Vec3, expected: Vec3) {
        assert!(
            actual.abs_diff_eq(expected, EPS),
            "expected {expected:?}, got {actual:?}"
        );
    }

    /// Camera at the origin looking down -Z. With vfov = 90 and aspect = 2 the
    /// view plane is 4 wide and 2 tall at z = -1, so the corners are easy to
    /// work out by hand:
    ///   lower_left  = (-2, -1, -1)   lower_right = ( 2, -1, -1)
    ///   upper_left  = (-2,  1, -1)   upper_right = ( 2,  1, -1)
    fn simple_camera() -> Camera {
        Camera::new(Vec3::ZERO, vec3(0.0, 0.0, -1.0), Vec3::Y, 90.0, 2.0)
    }

    #[test]
    fn basis_for_simple_camera() {
        let (u, v, w) = simple_camera().basis();
        assert_vec_eq(u, Vec3::X);
        assert_vec_eq(v, Vec3::Y);
        assert_vec_eq(w, Vec3::Z);
    }

    #[test]
    fn basis_is_orthonormal_and_right_handed() {
        // the camera from the lecture slides
        let cam = Camera::new(
            vec3(-0.5, 3.0, 4.4),
            vec3(0.0, 0.6, 0.0),
            Vec3::Y,
            60.0,
            16.0 / 9.0,
        );
        let (u, v, w) = cam.basis();
        for axis in [u, v, w] {
            assert!((axis.length() - 1.0).abs() < EPS, "{axis:?} is not unit");
        }
        assert!(u.dot(v).abs() < EPS);
        assert!(v.dot(w).abs() < EPS);
        assert!(w.dot(u).abs() < EPS);
        assert_vec_eq(u.cross(v), w); // right-handed
        assert!(v.y > 0.0, "camera up should lean toward world up");
    }

    #[test]
    fn corner_rays_simple_camera() {
        let cam = simple_camera();
        let cases = [
            ((0.0, 0.0), vec3(-2.0, -1.0, -1.0)), // lower-left
            ((1.0, 0.0), vec3(2.0, -1.0, -1.0)),  // lower-right
            ((0.0, 1.0), vec3(-2.0, 1.0, -1.0)),  // upper-left
            ((1.0, 1.0), vec3(2.0, 1.0, -1.0)),   // upper-right
            ((0.5, 0.5), vec3(0.0, 0.0, -1.0)),   // center
        ];
        for ((s, t), expected_dir) in cases {
            let r = cam.ray(s, t);
            assert_vec_eq(r.origin(), Vec3::ZERO);
            assert_vec_eq(r.direction(), expected_dir);
        }
    }

    #[test]
    fn corner_rays_camera_on_x_axis() {
        // Eye at (5, 0, 0) looking back at the origin.
        //   w = (1, 0, 0), u = Y x w = (0, 0, -1), v = w x u = (0, 1, 0)
        //   horizontal = 4u = (0, 0, -4), vertical = 2v = (0, 2, 0)
        //   lower_left = eye - 2u - v - w = (4, -1, 2)
        let eye = vec3(5.0, 0.0, 0.0);
        let cam = Camera::new(eye, Vec3::ZERO, Vec3::Y, 90.0, 2.0);

        let (u, v, w) = cam.basis();
        assert_vec_eq(u, vec3(0.0, 0.0, -1.0));
        assert_vec_eq(v, Vec3::Y);
        assert_vec_eq(w, Vec3::X);

        let cases = [
            ((0.0, 0.0), vec3(-1.0, -1.0, 2.0)),
            ((1.0, 0.0), vec3(-1.0, -1.0, -2.0)),
            ((0.0, 1.0), vec3(-1.0, 1.0, 2.0)),
            ((1.0, 1.0), vec3(-1.0, 1.0, -2.0)),
            ((0.5, 0.5), vec3(-1.0, 0.0, 0.0)),
        ];
        for ((s, t), expected_dir) in cases {
            let r = cam.ray(s, t);
            assert_vec_eq(r.origin(), eye);
            assert_vec_eq(r.direction(), expected_dir);
        }
    }

    #[test]
    fn center_ray_points_at_target() {
        let eye = vec3(-0.5, 3.0, 4.4);
        let target = vec3(0.0, 0.6, 0.0);
        let cam = Camera::new(eye, target, Vec3::Y, 60.0, 16.0 / 9.0);
        let r = cam.ray(0.5, 0.5);
        assert_vec_eq(r.origin(), eye);
        assert_vec_eq(r.direction().normalize(), (target - eye).normalize());
    }
}
