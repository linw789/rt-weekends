use crate::camera::Camera;
use crate::materials::{Material, MaterialDielectric, MaterialDiffuse, MaterialMetal};
use crate::shapes::{Aabb, Quad, Ray, RayIntersection, Shape, Sphere};
use crate::types::Fp;
use crate::vecmath::{Color3F, Vec3F};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::ops::Range;
use std::path::Path;
use std::sync::Arc;
use std::vec::Vec;

pub struct BvhLeaf {
    pub aabb: Aabb,
    pub shape_index: usize,
}

pub struct BvhLink {
    pub aabb: Aabb,
    pub left: Arc<BvhNode>,
    pub right: Arc<BvhNode>,
}

pub enum BvhNode {
    Leaf(BvhLeaf),
    Link(BvhLink),
}

impl BvhNode {
    pub fn aabb(&self) -> &Aabb {
        match self {
            BvhNode::Leaf(leaf) => &leaf.aabb,
            BvhNode::Link(link) => &link.aabb,
        }
    }
}

fn build_bvh(shapes: &[Shape], shape_start: usize) -> Arc<BvhNode> {
    if shapes.len() == 1 {
        Arc::new(BvhNode::Leaf(BvhLeaf {
            aabb: shapes[0].calc_aabb(),
            shape_index: shape_start,
        }))
    } else {
        let middle = shapes.len() / 2;
        let left = build_bvh(&shapes[..middle], shape_start);
        let right = build_bvh(&shapes[middle..], shape_start + middle);

        Arc::new(BvhNode::Link(BvhLink {
            aabb: Aabb::merge(&left.aabb(), &right.aabb()),
            left,
            right,
        }))
    }
}

fn bvh_ray_intersect(
    bvh_node: Arc<BvhNode>,
    shapes: &[Shape],
    ray: &Ray,
    limits: &Range<Fp>,
) -> (RayIntersection, usize) {
    if bvh_node.aabb().ray_intersect(ray) {
        match &*bvh_node {
            BvhNode::Leaf(leaf) => (
                shapes[leaf.shape_index].ray_intersect(ray, limits),
                leaf.shape_index,
            ),
            BvhNode::Link(link) => {
                let (left_intersection, left_sphere_index) =
                    bvh_ray_intersect(Arc::clone(&link.left), shapes, ray, limits);
                let right_limits = limits.start..if left_intersection.hit {
                    left_intersection.t
                } else {
                    limits.end
                };
                let (right_intersection, right_sphere_index) =
                    bvh_ray_intersect(Arc::clone(&link.right), shapes, ray, &right_limits);
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
            0,
        )
    }
}

pub struct Scene {
    shapes: Vec<Shape>,
    bvh: Arc<BvhNode>,
}

impl Scene {
    const TRACE_MAX_DEPTH: u32 = 5;

    #[allow(dead_code)]
    pub fn one_sphere() -> Self {
        let mut shapes = Vec::new();

        shapes.push(Shape::Sphere(Sphere::new(
            Vec3F::new(0.0, 0.0, -1.0),
            0.5,
            Material::Diffuse(MaterialDiffuse::new_solid_color(Color3F::new(
                0.7, 0.3, 0.3,
            ))),
        )));

        Self {
            bvh: build_bvh(&shapes, 0),
            shapes,
        }
    }

    #[allow(dead_code)]
    pub fn two_spheres() -> Scene {
        let shapes = vec![
            Shape::Sphere(Sphere::new(
                Vec3F::new(0.0, -100.5, -1.0),
                100.0,
                Material::Diffuse(MaterialDiffuse::new_solid_color(Color3F::new(
                    0.7, 0.3, 0.3,
                ))),
            )),
            Shape::Sphere(Sphere::new(
                Vec3F::new(0.0, 0.0, -1.0),
                0.5,
                Material::Diffuse(MaterialDiffuse::new_solid_color(Color3F::new(
                    0.8, 0.6, 0.2,
                ))),
            )),
        ];

        Self {
            bvh: build_bvh(&shapes, 0),
            shapes,
        }
    }

    #[allow(dead_code)]
    pub fn two_globes() -> Scene {
        let globes = vec![
            Shape::Sphere(Sphere::new(
                Vec3F::new(0.0, -100.5, -1.0),
                100.0,
                Material::Diffuse(MaterialDiffuse::from_image(Path::new(
                    "images/earthmap.jpg",
                ))),
            )),
            Shape::Sphere(Sphere::new(
                Vec3F::new(0.0, 100.5, -1.0),
                100.0,
                Material::Diffuse(MaterialDiffuse::from_image(Path::new(
                    "images/earthmap.jpg",
                ))),
            )),
        ];

        Self {
            bvh: build_bvh(&globes, 0),
            shapes: globes,
        }
    }

    #[allow(dead_code)]
    pub fn three_spheres_metal() -> Scene {
        let shapes = vec![
            // ground
            Shape::Sphere(Sphere::new(
                Vec3F::new(0.0, -100.5, -1.0),
                100.0,
                Material::Diffuse(MaterialDiffuse::new_checker(
                    Color3F::new(0.2, 0.3, 0.1),
                    Color3F::new(0.9, 0.9, 0.9),
                    0.32,
                )),
            )),
            // center
            Shape::Sphere(Sphere::new(
                Vec3F::new(0.0, 0.0, -1.2),
                0.5,
                Material::Diffuse(MaterialDiffuse::new_solid_color(Color3F::new(
                    0.1, 0.2, 0.5,
                ))),
            )),
            // left
            Shape::Sphere(Sphere::new(
                Vec3F::new(-1.0, 0.0, -1.0),
                0.5,
                Material::Metal(MaterialMetal::new(Color3F::new(0.8, 0.8, 0.8), 0.3)),
            )),
            // right
            Shape::Sphere(Sphere::new(
                Vec3F::new(1.0, 0.0, -1.0),
                0.5,
                Material::Metal(MaterialMetal::new(Color3F::new(0.8, 0.6, 0.2), 1.0)),
            )),
        ];

        Self {
            bvh: build_bvh(&shapes, 0),
            shapes,
        }
    }

    #[allow(dead_code)]
    pub fn three_spheres_dielectric() -> Scene {
        let shapes = vec![
            // ground
            Shape::Sphere(Sphere::new(
                Vec3F::new(0.0, -100.5, -1.0),
                100.0,
                Material::Diffuse(MaterialDiffuse::new_solid_color(Color3F::new(
                    0.8, 0.8, 0.0,
                ))),
            )),
            // center
            Shape::Sphere(Sphere::new(
                Vec3F::new(0.0, 0.0, -1.2),
                0.5,
                Material::Diffuse(MaterialDiffuse::new_solid_color(Color3F::new(
                    0.1, 0.2, 0.5,
                ))),
            )),
            // left
            Shape::Sphere(Sphere::new(
                Vec3F::new(-1.0, 0.0, -1.0),
                0.5,
                Material::Dielectric(MaterialDielectric::new(1.0 / 1.333)),
            )),
            // right
            Shape::Sphere(Sphere::new(
                Vec3F::new(1.0, 0.0, -1.0),
                0.5,
                Material::Metal(MaterialMetal::new(Color3F::new(0.8, 0.6, 0.2), 1.0)),
            )),
        ];

        Self {
            bvh: build_bvh(&shapes, 0),
            shapes,
        }
    }

    #[allow(dead_code)]
    pub fn three_spheres_hollow_glass() -> Scene {
        let shapes = vec![
            // ground
            Shape::Sphere(Sphere::new(
                Vec3F::new(0.0, -100.5, -1.0),
                100.0,
                Material::Diffuse(MaterialDiffuse::new_solid_color(Color3F::new(
                    0.8, 0.8, 0.0,
                ))),
            )),
            // center
            Shape::Sphere(Sphere::new(
                Vec3F::new(0.0, 0.0, -1.2),
                0.5,
                Material::Diffuse(MaterialDiffuse::new_solid_color(Color3F::new(
                    0.1, 0.2, 0.5,
                ))),
            )),
            // left
            Shape::Sphere(Sphere::new(
                Vec3F::new(-1.0, 0.0, -1.0),
                0.5,
                Material::Dielectric(MaterialDielectric::new(1.5)),
            )),
            // air bubble inside the left glass sphere
            Shape::Sphere(Sphere::new(
                Vec3F::new(-1.0, 0.0, -1.0),
                0.4,
                Material::Dielectric(MaterialDielectric::new(1.0 / 1.5)),
            )),
            // right
            Shape::Sphere(Sphere::new(
                Vec3F::new(1.0, 0.0, -1.0),
                0.5,
                Material::Metal(MaterialMetal::new(Color3F::new(0.8, 0.6, 0.2), 1.0)),
            )),
        ];

        Self {
            bvh: build_bvh(&shapes, 0),
            shapes,
        }
    }

    #[allow(dead_code)]
    pub fn many_spheres() -> Scene {
        let mut shapes = vec![
            // ground
            Shape::Sphere(Sphere::new(
                Vec3F::new(0.0, -1000.0, 0.0),
                1000.0,
                Material::Diffuse(MaterialDiffuse::new_solid_color(Color3F::new(
                    0.5, 0.5, 0.5,
                ))),
            )),
            Shape::Sphere(Sphere::new(
                Vec3F::new(0.0, 1.0, 0.0),
                1.0,
                Material::Dielectric(MaterialDielectric::new(1.5)),
            )),
            Shape::Sphere(Sphere::new(
                Vec3F::new(-4.0, 1.0, 0.0),
                1.0,
                Material::Diffuse(MaterialDiffuse::new_solid_color(Color3F::new(
                    0.4, 0.2, 0.1,
                ))),
            )),
            Shape::Sphere(Sphere::new(
                Vec3F::new(4.0, 1.0, 0.0),
                1.0,
                Material::Metal(MaterialMetal::new(Color3F::new(0.7, 0.6, 0.5), 0.0)),
            )),
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
                        shapes.push(Shape::Sphere(Sphere::new(
                            pos,
                            0.2,
                            Material::Diffuse(MaterialDiffuse::new_solid_color(albedo)),
                        )));
                    } else if choose_material < 0.9 {
                        let albedo = Color3F::new(
                            rand.gen_range(0.5..1.0),
                            rand.gen_range(0.5..1.0),
                            rand.gen_range(0.5..1.0),
                        );
                        let fuzz = rand.gen_range(0.0..0.5);
                        shapes.push(Shape::Sphere(Sphere::new(
                            pos,
                            0.2,
                            Material::Metal(MaterialMetal::new(albedo, fuzz)),
                        )));
                    } else {
                        shapes.push(Shape::Sphere(Sphere::new(
                            pos,
                            0.2,
                            Material::Dielectric(MaterialDielectric::new(1.5)),
                        )));
                    }
                }
            }
        }

        Self {
            bvh: build_bvh(&shapes, 0),
            shapes,
        }
    }

    #[allow(dead_code)]
    pub fn quads_example() -> Scene {
        let shapes = vec![
            // left red
            Shape::Quad(Quad::new(
                Vec3F::new(-3.0, -2.0, 5.0),
                Vec3F::new(0.0, 0.0, -4.0),
                Vec3F::new(0.0, 4.0, 0.0),
                Material::Diffuse(MaterialDiffuse::new_solid_color(Color3F::new(
                    1.0, 0.2, 0.2,
                ))),
            )),
            // back green
            Shape::Quad(Quad::new(
                Vec3F::new(-2.0, -2.0, 0.0),
                Vec3F::new(4.0, 0.0, 0.0),
                Vec3F::new(0.0, 4.0, 0.0),
                Material::Diffuse(MaterialDiffuse::new_solid_color(Color3F::new(
                    0.2, 1.0, 0.2,
                ))),
            )),
            // right blue
            Shape::Quad(Quad::new(
                Vec3F::new(3.0, -2.0, 1.0),
                Vec3F::new(0.0, 0.0, 4.0),
                Vec3F::new(0.0, 4.0, 0.0),
                Material::Diffuse(MaterialDiffuse::new_solid_color(Color3F::new(
                    0.2, 0.2, 1.0,
                ))),
            )),
            // upper orange
            Shape::Quad(Quad::new(
                Vec3F::new(-2.0, 3.0, 1.0),
                Vec3F::new(4.0, 0.0, 0.0),
                Vec3F::new(0.0, 0.0, 4.0),
                Material::Diffuse(MaterialDiffuse::new_solid_color(Color3F::new(
                    1.0, 0.5, 0.0,
                ))),
            )),
            // lower teal
            Shape::Quad(Quad::new(
                Vec3F::new(-2.0, -3.0, 5.0),
                Vec3F::new(4.0, 0.0, 0.0),
                Vec3F::new(0.0, 0.0, -4.0),
                Material::Diffuse(MaterialDiffuse::new_solid_color(Color3F::new(
                    0.2, 0.2, 0.8,
                ))),
            )),
        ];

        Self {
            bvh: build_bvh(&shapes, 0),
            shapes,
        }
    }

    #[allow(dead_code)]
    pub fn quads_example_camera(image_width: u32, image_height: u32) -> Camera {
        Camera::builder()
            .pixel_dimension(image_width, image_height)
            .fov(80.0 / 180.0)
            .focus_length(10.0)
            .defocus_angle(0.0)
            .position(Vec3F::new(0.0, 0.0, 9.0))
            .lookat(Vec3F::zero())
            .up(Vec3F::new(0.0, 1.0, 0.0))
            .build()
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

        for shape in self.shapes.iter() {
            let limits = 0.001..Fp::MAX;
            let intersection = shape.ray_intersect(ray, &limits);
            if intersection.hit && intersection.t < nearest_intersection.t {
                nearest_intersection = intersection;
                nearest_material = Some(shape.get_material());
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
        let (intersection, sphere_index) =
            bvh_ray_intersect(Arc::clone(&self.bvh), &self.shapes, &ray, &limits);

        let color = if intersection.hit {
            let material = &self.shapes[sphere_index].get_material();
            match material.scatter(ray, &intersection, rand) {
                Some((scattered_ray, albedo)) => {
                    albedo * self.trace_bvh(&scattered_ray, rand, depth + 1)
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
