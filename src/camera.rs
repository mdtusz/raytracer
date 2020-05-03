use rand::prelude::*;

use ultraviolet::{Mat4, Vec3, Vec4};

use crate::ray::Ray;

pub struct Camera {
    focus_distance: f32,
    lens_radius: f32,
    position: Vec3,
    rotation: Mat4,
    scaling: Vec4,
    w: f32,
    open: f32,
    close: f32,
}

impl Camera {
    pub fn new(
        position: Vec3,
        look_at: Vec3,
        aspect: f32,
        fov: f32,
        focus_distance: f32,
        aperture: f32,
        open: f32,
        close: f32,
    ) -> Self {
        let w = -1.0 / (fov / 2.0).tan();
        let scaling = Vec4::new(aspect, 1.0, 1.0, 1.0);
        let rot = Mat4::look_at(position, look_at, Vec3::new(0.0, 1.0, 0.0));

        Self {
            focus_distance,
            lens_radius: aperture / 2.0,
            position,
            rotation: rot.inversed(),
            scaling,
            w,
            open,
            close,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let uvw = (Vec4::new(u, v, self.w, 0.0) * self.scaling).normalized() * self.focus_distance;
        let rd = self.rotation * uvw;

        let dof_offset = random_in_unit_disk() * self.lens_radius;
        let offset = self.rotation * Vec4::new(dof_offset.x, dof_offset.y, 0.0, 0.0);

        let time = thread_rng().gen_range(self.open, self.close);
        Ray::new(self.position + offset.xyz(), rd.xyz() - offset.xyz(), time)
    }
}

fn random_in_unit_disk() -> Vec3 {
    let mut rng = thread_rng();
    loop {
        let p = Vec3::new(rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0), 0.0);

        if p.mag() < 1.0 {
            return p;
        }
    }
}
