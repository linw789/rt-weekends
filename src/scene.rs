use crate::materials::{Material, MaterialDiffuse};
use crate::shapes::{Ray, RayInterception, Sphere};
use crate::types::Fp;
use crate::vecmath::{Color3F, Vec3F};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::vec::Vec;

pub struct Scene {
    spheres: Vec<Sphere>,
}

impl Scene {
    pub fn from_file(filepath: &Path) -> io::Result<Scene> {
        let file = File::open(filepath)?;

        let mut scene = Scene {
            spheres: Vec::new(),
        };

        let mut is_parsing_sphere = false;
        let mut is_parsing_material = false;
        let mut sphere_position: Option<Vec3F> = None;
        let mut sphere_radius: Option<Fp> = None;

        for line in BufReader::new(file).lines() {
            let line = line.unwrap();
            let line_trimmed = line.trim_start();
            if is_parsing_sphere {
                if line_trimmed.starts_with("position") {
                    let mut pos_components =
                        line_trimmed.trim_start_matches("position:").split(",");
                    sphere_position = Some(Vec3F::new(
                        pos_components.next().unwrap().trim().parse::<Fp>().unwrap(),
                        pos_components.next().unwrap().trim().parse::<Fp>().unwrap(),
                        pos_components.next().unwrap().trim().parse::<Fp>().unwrap(),
                    ));
                } else if line_trimmed.starts_with("radius") {
                    let radius = line_trimmed.trim_start_matches("radius:");
                    sphere_radius = Some(radius.trim().parse::<Fp>().unwrap());
                } else {
                    return Result::Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!("parsing failed: '{}'", line),
                    ));
                }

                match (sphere_position, sphere_radius) {
                    (Some(pos), Some(radius)) => {
                        // scene.spheres.push(Sphere::new(pos, radius));
                        // sphere_position = None;
                        // sphere_radius = None;
                        // is_parsing_sphere = false;
                    }
                    _ => { /* do nothing */ }
                }
            } else if is_parsing_material {
                if line_trimmed.starts_with("type:") {
                    let type_line = line_trimmed.trim_start_matches("type:").trim();
                    if type_line == "diffuse" {
                    } else {
                        panic!("to do");
                    }
                } else {
                    return Result::Err(
                        io::Error::new(
                            io::ErrorKind::Other,
                            format!("parsing failed: the first field is not 'type:' under 'material:'. '{}'", line)));
                }
            } else {
                if line_trimmed.starts_with("sphere:") {
                    is_parsing_sphere = true;
                } else if line_trimmed.starts_with("material:") {
                    is_parsing_material = true;
                } else {
                    return Result::Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!("parsing failed: '{}'", line),
                    ));
                }
            }
        }

        Result::Ok(scene)
    }

    pub fn one_sphere() -> Scene {
        let mut scene = Scene {
            spheres: Vec::new(),
        };

        scene.spheres.push(Sphere::new(
            Vec3F::new(0.0, 0.0, -1.0),
            0.5,
            Material::Diffuse(MaterialDiffuse::new(Vec3F::new(0.7, 0.3, 0.3))),
        ));

        scene
    }

    pub fn two_spheres() -> Scene {
        let mut scene = Scene {
            spheres: Vec::new(),
        };

        scene.spheres.push(Sphere::new(
            Vec3F::new(0.0, -100.5, -1.0),
            100.0,
            Material::Diffuse(MaterialDiffuse::new(Vec3F::new(0.7, 0.3, 0.3))),
        ));

        scene.spheres.push(Sphere::new(
            Vec3F::new(0.0, 0.0, -1.0),
            0.5,
            Material::Diffuse(MaterialDiffuse::new(Vec3F::new(0.8, 0.6, 0.2))),
        ));

        scene
    }

    pub fn trace(&self, ray: &Ray) -> Color3F {
        let mut hit = false;
        for sphere in self.spheres.iter() {
            let ray_interception = sphere.ray_intercept(ray);
            if ray_interception.hit {
                hit = true;
            }
        }

        if hit {
            Color3F::new(0.7, 0.3, 0.3)
        } else {
            Color3F::new(0.0, 1.0, 0.0)
        }
    }
}
