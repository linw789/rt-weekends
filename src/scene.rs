use crate::materials::{Material, MaterialDielectric, MaterialDiffuse, MaterialMetal};
use crate::shapes::{Ray, RayIntersection, Sphere};
use crate::types::Fp;
use crate::vecmath::{Color3F, Vec3F};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::vec::Vec;

pub struct Scene {
    spheres: Vec<Sphere>,
}

impl Scene {
    const TRACE_MAX_DEPTH: u32 = 50;

    #[allow(dead_code)]
    pub fn one_sphere() -> Scene {
        let mut scene = Scene {
            spheres: Vec::new(),
        };

        scene.spheres.push(Sphere::new(
            Vec3F::new(0.0, 0.0, -1.0),
            0.5,
            Material::Diffuse(MaterialDiffuse::new(Color3F::new(0.7, 0.3, 0.3))),
        ));

        scene
    }

    #[allow(dead_code)]
    pub fn two_spheres() -> Scene {
        let scene = Scene {
            spheres: vec![
                Sphere::new(
                    Vec3F::new(0.0, -100.5, -1.0),
                    100.0,
                    Material::Diffuse(MaterialDiffuse::new(Color3F::new(0.7, 0.3, 0.3))),
                ),
                Sphere::new(
                    Vec3F::new(0.0, 0.0, -1.0),
                    0.5,
                    Material::Diffuse(MaterialDiffuse::new(Color3F::new(0.8, 0.6, 0.2))),
                ),
            ],
        };

        scene
    }

    #[allow(dead_code)]
    pub fn three_spheres_metal() -> Scene {
        let scene = Scene {
            spheres: vec![
                // ground
                Sphere::new(
                    Vec3F::new(0.0, -100.5, -1.0),
                    100.0,
                    Material::Diffuse(MaterialDiffuse::new(Color3F::new(0.8, 0.8, 0.0))),
                ),
                // center
                Sphere::new(
                    Vec3F::new(0.0, 0.0, -1.2),
                    0.5,
                    Material::Diffuse(MaterialDiffuse::new(Color3F::new(0.1, 0.2, 0.5))),
                ),
                // left
                Sphere::new(
                    Vec3F::new(-1.0, 0.0, -1.0),
                    0.5,
                    Material::Metal(MaterialMetal::new(Color3F::new(0.8, 0.8, 0.8), 0.3)),
                ),
                // right
                Sphere::new(
                    Vec3F::new(1.0, 0.0, -1.0),
                    0.5,
                    Material::Metal(MaterialMetal::new(Color3F::new(0.8, 0.6, 0.2), 1.0)),
                ),
            ],
        };

        scene
    }

    #[allow(dead_code)]
    pub fn three_spheres_dielectric() -> Scene {
        let scene = Scene {
            spheres: vec![
                // ground
                Sphere::new(
                    Vec3F::new(0.0, -100.5, -1.0),
                    100.0,
                    Material::Diffuse(MaterialDiffuse::new(Color3F::new(0.8, 0.8, 0.0))),
                ),
                // center
                Sphere::new(
                    Vec3F::new(0.0, 0.0, -1.2),
                    0.5,
                    Material::Diffuse(MaterialDiffuse::new(Color3F::new(0.1, 0.2, 0.5))),
                ),
                // left
                Sphere::new(
                    Vec3F::new(-1.0, 0.0, -1.0),
                    0.5,
                    Material::Dielectric(MaterialDielectric::new(1.0 / 1.333)),
                ),
                // right
                Sphere::new(
                    Vec3F::new(1.0, 0.0, -1.0),
                    0.5,
                    Material::Metal(MaterialMetal::new(Color3F::new(0.8, 0.6, 0.2), 1.0)),
                ),
            ],
        };

        scene
    }

    #[allow(dead_code)]
    pub fn three_spheres_hollow_glass() -> Scene {
        let scene = Scene {
            spheres: vec![
                // ground
                Sphere::new(
                    Vec3F::new(0.0, -100.5, -1.0),
                    100.0,
                    Material::Diffuse(MaterialDiffuse::new(Color3F::new(0.8, 0.8, 0.0))),
                ),
                // center
                Sphere::new(
                    Vec3F::new(0.0, 0.0, -1.2),
                    0.5,
                    Material::Diffuse(MaterialDiffuse::new(Color3F::new(0.1, 0.2, 0.5))),
                ),
                // left
                Sphere::new(
                    Vec3F::new(-1.0, 0.0, -1.0),
                    0.5,
                    Material::Dielectric(MaterialDielectric::new(1.5)),
                ),
                // air bubble inside the left glass sphere
                Sphere::new(
                    Vec3F::new(-1.0, 0.0, -1.0),
                    0.4,
                    Material::Dielectric(MaterialDielectric::new(1.0 / 1.5)),
                ),
                // right
                Sphere::new(
                    Vec3F::new(1.0, 0.0, -1.0),
                    0.5,
                    Material::Metal(MaterialMetal::new(Color3F::new(0.8, 0.6, 0.2), 1.0)),
                ),
            ],
        };

        scene
    }

    #[allow(dead_code)]
    pub fn many_spheres() -> Scene {
        let mut spheres = vec![
            // ground
            Sphere::new(
                Vec3F::new(0.0, -1000.0, 0.0),
                1000.0,
                Material::Diffuse(MaterialDiffuse::new(Color3F::new(0.5, 0.5, 0.5))),
            ),
            Sphere::new(
                Vec3F::new(0.0, 1.0, 0.0),
                1.0,
                Material::Dielectric(MaterialDielectric::new(1.5)),
            ),
            Sphere::new(
                Vec3F::new(-4.0, 1.0, 0.0),
                1.0,
                Material::Diffuse(MaterialDiffuse::new(Color3F::new(0.4, 0.2, 0.1))),
            ),
            Sphere::new(
                Vec3F::new(4.0, 1.0, 0.0),
                1.0,
                Material::Metal(MaterialMetal::new(Color3F::new(0.7, 0.6, 0.5), 0.0)),
            ),
        ];

        let mut rand = SmallRng::seed_from_u64(877);

        for x in -11..11 {
            for y in -11..11 {
                let pos = Vec3F::new(
                    x as Fp + 0.9 * rand.gen_range(0.0..1.0),
                    0.2,
                    y as Fp + 0.9 * rand.gen_range(0.0..1.0));

                if (pos - Vec3F::new(4.0, 0.2, 0.0)).length() > 0.9 {
                    let choose_material = rand.gen_range(0.0..1.0);
                    if choose_material < 0.6 {
                        let albedo = Color3F::new(rand.gen_range(0.0..1.0), rand.gen_range(0.0..1.0), rand.gen_range(0.0..1.0));
                        spheres.push(
                            Sphere::new(pos, 0.2, Material::Diffuse(MaterialDiffuse::new(albedo))));
                    } else if choose_material < 0.9 {
                        let albedo = Color3F::new(rand.gen_range(0.5..1.0), rand.gen_range(0.5..1.0), rand.gen_range(0.5..1.0));
                        let fuzz = rand.gen_range(0.0..0.5);
                        spheres.push(
                            Sphere::new(pos, 0.2, Material::Metal(MaterialMetal::new(albedo, fuzz))));
                    } else {
                        spheres.push(
                            Sphere::new(pos, 0.2, Material::Dielectric(MaterialDielectric::new(1.5))));
                    }
                }
            }
        }

        Scene { spheres }
    }

    pub fn trace<R: rand::Rng>(&self, ray: &Ray, rand: &mut R, depth: u32) -> Color3F {
        if depth > Self::TRACE_MAX_DEPTH {
            return Color3F::zero();
        }

        let mut nearest_intersection = RayIntersection {
            t: Fp::MAX,
            ..Default::default()
        };
        let mut nearest_material: Option<&Material> = None;

        for sphere in self.spheres.iter() {
            let limits = 0.001..Fp::MAX;
            let intersection = sphere.ray_intercept(ray, &limits);
            if intersection.hit && intersection.t < nearest_intersection.t {
                nearest_intersection = intersection;
                nearest_material = Some(&sphere.material);
            }
        }

        let color = if nearest_intersection.hit {
            let material = nearest_material.unwrap();
            match material.scatter(ray, &nearest_intersection, rand) {
                Some((scattered_ray, albedo)) => {
                    albedo * self.trace(&scattered_ray, rand, depth + 1)
                }
                None => Color3F::zero(),
            }
        } else {
            // simulate the sky color
            let ray_dir_normalized = ray.direction.normalized();
            let a = 0.5 * (ray_dir_normalized.y + 1.0);
            Color3F::new(1.0, 1.0, 1.0) * (1.0 - a) + Color3F::new(0.5, 0.7, 1.0) * a
        };

        color
    }
}
