pub mod color;
pub mod vec3;
pub mod ray;
pub mod hittable;
pub mod sphere;
pub mod hittable_list;
pub mod commun;
pub mod camera;
 
pub use std::io;
 
pub use color::Color;
pub use camera::Camera;


pub use ray::Ray;
pub use vec3::{Point3, Vec3};
pub use hittable::{HitRecord, Hittable};
pub use hittable_list::HittableList;
pub use sphere::Sphere;


pub fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin() - center;
    let a = r.direction().length_squared();
    let half_b = vec3::dot(oc, r.direction());
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - f64::sqrt(discriminant)) / a
    }
}

/* fn hit_plane(y: f64, r: &Ray) -> bool {
    let t = (y - r.origin().z()) / r.direction().z();
    t > 0.0
} */

// This function returns the color of the background (e.g., a simple gradient).
pub fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    let mut rec = HitRecord::new();
    if world.hit(r, 0.0, commun::INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }

    /* if hit_plane(-1.0, r) {
        return Color::new(1.0, 1.0, 1.0); // Vert pour le plan
    } */

    let unit_direction = vec3::unit_vector(r.direction());
    let _t = 0.5 * (unit_direction.y() + 1.0);
    // (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    // (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0) + t * Color::new(0.0, 0.0, 0.0)
    Color::new(1.0, 1.0, 1.0) 
}