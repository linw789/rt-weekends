use crate::materials::{Material, MaterialDielectric, MaterialDiffuse, MaterialMetal};
use crate::shapes::{Aabb, Ray, RayIntersection, Sphere};
use crate::types::Fp;
use crate::vecmath::{Color3F, Vec3F};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::vec::Vec;
use std::ops::Range;
use std::rc::Rc;

pub struct BvhLeaf {
    pub aabb: Aabb,
    pub sphere_index: usize,
}

pub struct BvhLink {
    pub aabb: Aabb,
    pub left: Rc<BvhNode>,
    pub right: Rc<BvhNode>,
}

pub enum BvhNode {
    Leaf(BvhLeaf),
    Link(BvhLink),
}

impl BvhNode {
    pub fn aabb(&self) -> Aabb {
        match self {
            BvhNode::Leaf(leaf) => leaf.aabb,
            BvhNode::Link(link) => link.aabb,
        }
    }
}

fn build_bvh(spheres: &[Sphere], sphere_start: usize) -> Rc<BvhNode> {
    if spheres.len() == 1 {
        Rc::new(BvhNode::Leaf(BvhLeaf {
            aabb: Aabb::from_sphere(&spheres[0]),
            sphere_index: sphere_start,
        }))
    } else {
        let middle = spheres.len() / 2;
        let left = build_bvh(&spheres[..middle], sphere_start);
        let right = build_bvh(&spheres[middle..], sphere_start + middle);

        Rc::new(BvhNode::Link(BvhLink {
            aabb: Aabb::merge(&left.aabb(), &right.aabb()),
            left,
            right
        }))
    }
}

fn bvh_ray_intersect(bvh_node: Rc<BvhNode>, spheres: &[Sphere], ray: &Ray, limits: &Range<Fp>) -> (RayIntersection, usize) {
    if bvh_node.aabb().ray_intersect(ray) {
        match &*bvh_node {
            BvhNode::Leaf(leaf) => (spheres[leaf.sphere_index].ray_intersect(ray, limits), leaf.sphere_index),
            BvhNode::Link(link) => {
                let (left_intersection, left_sphere_index) = bvh_ray_intersect(Rc::clone(&link.left), spheres, ray, limits);
                let right_limits = limits.start .. if left_intersection.hit { left_intersection.t } else { limits.end };
                let (right_intersection, right_sphere_index) = bvh_ray_intersect(Rc::clone(&link.right), spheres, ray, &right_limits);
                if right_intersection.hit { 
                    (right_intersection, right_sphere_index)
                } else {
                    (left_intersection, left_sphere_index)
                }
            }
        }
    } else {
        (
            RayIntersection {
            hit: false,
            ..Default::default()
            },
            0
        )
    }
}

pub struct Scene {
    spheres: Vec<Sphere>,
    bvh: Rc<BvhNode>,
}

impl Scene {
    const TRACE_MAX_DEPTH: u32 = 50;

    #[allow(dead_code)]
    pub fn one_sphere() -> Self {
        let mut spheres = Vec::new();

        spheres.push(Sphere::new(
            Vec3F::new(0.0, 0.0, -1.0),
            0.5,
            Material::Diffuse(MaterialDiffuse::new(Color3F::new(0.7, 0.3, 0.3))),
        ));

        Self {
            bvh: build_bvh(&spheres, 0),
            spheres,
        }
    }

    #[allow(dead_code)]
    pub fn two_spheres() -> Scene {
        let spheres = vec![
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
        ];

        Self {
            bvh: build_bvh(&spheres, 0),
            spheres,
        }
    }

    #[allow(dead_code)]
    pub fn three_spheres_metal() -> Scene {
        let spheres = vec![
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
        ];

        Self {
            bvh: build_bvh(&spheres, 0),
            spheres,
        }
    }

    #[allow(dead_code)]
    pub fn three_spheres_dielectric() -> Scene {
        let spheres = vec![
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
        ];

        Self {
            bvh: build_bvh(&spheres, 0),
            spheres,
        }
    }

    #[allow(dead_code)]
    pub fn three_spheres_hollow_glass() -> Scene {
        let spheres = vec![
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
        ];

        Self {
            bvh: build_bvh(&spheres, 0),
            spheres,
        }
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
                    y as Fp + 0.9 * rand.gen_range(0.0..1.0),
                );

                if (pos - Vec3F::new(4.0, 0.2, 0.0)).length() > 0.9 {
                    let choose_material = rand.gen_range(0.0..1.0);
                    if choose_material < 0.6 {
                        let albedo = Color3F::new(
                            rand.gen_range(0.0..1.0),
                            rand.gen_range(0.0..1.0),
                            rand.gen_range(0.0..1.0),
                        );
                        spheres.push(Sphere::new(
                            pos,
                            0.2,
                            Material::Diffuse(MaterialDiffuse::new(albedo)),
                        ));
                    } else if choose_material < 0.9 {
                        let albedo = Color3F::new(
                            rand.gen_range(0.5..1.0),
                            rand.gen_range(0.5..1.0),
                            rand.gen_range(0.5..1.0),
                        );
                        let fuzz = rand.gen_range(0.0..0.5);
                        spheres.push(Sphere::new(
                            pos,
                            0.2,
                            Material::Metal(MaterialMetal::new(albedo, fuzz)),
                        ));
                    } else {
                        spheres.push(Sphere::new(
                            pos,
                            0.2,
                            Material::Dielectric(MaterialDielectric::new(1.5)),
                        ));
                    }
                }
            }
        }

        Self {
            bvh: build_bvh(&spheres, 0),
            spheres,
        }
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
            let intersection = sphere.ray_intersect(ray, &limits);
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

    pub fn trace_bvh<R: rand::Rng>(&self, ray: &Ray, rand: &mut R, depth: u32) -> Color3F {
        if depth > Self::TRACE_MAX_DEPTH {
            return Color3F::zero();
        }

        let limits = 0.001..Fp::MAX;
        let (intersection, sphere_index) = bvh_ray_intersect(self.bvh.clone(), &self.spheres, &ray, &limits); 

        let color = if intersection.hit {
            let material = &self.spheres[sphere_index].material;
            match material.scatter(ray, &intersection, rand) {
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

unsafe impl Send for Scene {}
unsafe impl Sync for Scene {}
