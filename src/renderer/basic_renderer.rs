use super::Renderer;
use crate::tracer::{
    utils::{gamma_correct, in_range},
    Camera, HitableList, Ray, Vec3,
    pdf::{CosinePDF, HitablePDF, MixturePDF, PDF},
    textures::ConstantTexture,
    materials::DiffuseLight,
    objects::RectXZ, Hitable
};
use rand::Rng;

pub struct BasicRenderer<'a> {
    pub hitable_list: &'a HitableList,
    pub camera: &'a Camera,
    pub size: (u32, u32),
    pub anti_aliasing: u32,
    pub crop_region: ((u32, u32), (u32, u32)),
    pub ambient_light: bool,
}

impl BasicRenderer<'_> {
    fn color(&self, ray: &Ray, depth: u32) -> Vec3 {
        match self.hitable_list.hit(&ray, 0.001, std::f32::MAX) {
            Some(hit_record) => {
                let emitted = hit_record
                    .material
                    .emitted(&ray, &hit_record, hit_record.u, hit_record.v, hit_record.p);
                if depth > 0 {
                    match hit_record.material.scatter(&ray, &hit_record) {
                        Some((attenuation, scattered, pdf)) => {
                            /*
                            let mut rng = rand::thread_rng();
                            let on_light = Vec3::new(
                                rng.gen_range(213.0 ,343.0),
                                554.0,
                                rng.gen_range(227.0 ,332.0)
                            );
                            let to_light = on_light - hit_record.p;
                            let dist_squared = to_light.squared_length();
                            let to_light = to_light.unit();
                            if Vec3::dot(to_light, hit_record.normal) < 0.0 {
                                emitted
                            } else {
                                let light_area = ((343 - 213) * (332 - 227)) as f32;
                                let light_cosine = to_light.y.abs();
                                if light_cosine < 0.000001 {
                                    emitted
                                } else {
                                    let pdf = dist_squared / (light_cosine * light_area);
                                    let scattered = Ray::new(hit_record.p, to_light);
                                    emitted + Vec3::elemul(attenuation, self.color(&scattered, depth - 1)) 
                                              * (hit_record.material.scattering_pdf(&ray, &hit_record, &scattered) / pdf)
                                }

                            }
                            */
                            let light = DiffuseLight::new_arc(ConstantTexture::new(Vec3::new(15.0, 15.0, 15.0)));
                            let hitable = RectXZ::new(213.0, 343.0, 227.0, 332.0, 554.0, light) as Box<dyn Hitable>;
                            let p1 = HitablePDF::new(
                                hitable,
                                hit_record.p
                            );
                            let p2 = CosinePDF::new(hit_record.normal);
                            let p = MixturePDF::new(Box::new(p1) as Box<PDF>, Box::new(p2) as Box<PDF>);
                            let scattered = Ray::new(hit_record.p, p.generate());
                            let pdf = p.value(scattered.direction);
                            emitted + Vec3::elemul(attenuation, self.color(&scattered, depth - 1)) 
                                * (hit_record.material.scattering_pdf(&ray, &hit_record, &scattered) / pdf)
                        }
                        None => emitted,
                    }
                } else {
                    Vec3::zero()
                }
            }
            None => {
                if self.ambient_light {
                    let unit_direction = ray.direction.unit();
                    let t = 0.5 * (unit_direction.y + 1.0);
                    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
                } else {
                    Vec3::zero()
                }
            }
        }
    }
}

impl Renderer for BasicRenderer<'_> {
    fn render(&self) -> image::RgbaImage {
        let (render_width, render_height) = self.size;
        let ((crop_x, crop_y), (crop_width, crop_height)) = self.crop_region;
        let mut imgbuf = image::RgbaImage::new(crop_width, crop_height);
        let mut rng = rand::thread_rng();

        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let mut color = Vec3::zero();
            for _i in 0..self.anti_aliasing {
                let target_x: f32 = x as f32 + crop_x as f32 + rng.gen_range(0.0, 1.0);
                let u = target_x / render_width as f32;
                let target_y: f32 = y as f32 + crop_y as f32 + rng.gen_range(0.0, 1.0);
                let v = 1.0 - target_y / render_height as f32;
                let ray = self.camera.get_ray(u, v);
                color = color + self.color(&ray, 50);
            }
            *pixel = in_range(gamma_correct(color / self.anti_aliasing as f32)).rgba()
        }
        imgbuf
    }
}
