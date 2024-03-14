use crate::types::Fp;
use crate::vecmath::Vec3F;
use crate::shapes::Sphere;
use std::vec::Vec;
use std::fs::File;
use std::io::{BufReader, BufRead, self};
use std::path::Path;

pub struct Scene {
    spheres: Vec<Sphere>,
}

impl Scene {
    pub fn from_file(filepath: &Path) -> io::Result<Scene>  {
        let file = File::open(filepath)?;

        let mut scene = Scene { spheres: Vec::new(), };

        let mut is_parsing_sphere = false;
        let mut sphere_position: Option<Vec3F> = None;
        let mut sphere_radius: Option<Fp> = None;

        for line in BufReader::new(file).lines() {
            let line = line.unwrap();
            let line_trimmed = line.trim_start();
            if is_parsing_sphere {
                if line_trimmed.starts_with("position") {
                    let mut pos_components = line_trimmed.trim_start_matches("position:").split(",");
                    sphere_position = Some(Vec3F::new(
                        pos_components.next().unwrap().trim().parse::<Fp>().unwrap(),
                        pos_components.next().unwrap().trim().parse::<Fp>().unwrap(),
                        pos_components.next().unwrap().trim().parse::<Fp>().unwrap()));
                } else if line_trimmed.starts_with("radius") {
                    let radius = line_trimmed.trim_start_matches("radius:");
                    sphere_radius = Some(radius.trim().parse::<Fp>().unwrap());
                } else {
                    return Result::Err(
                        io::Error::new(
                            io::ErrorKind::Other,
                            format!("parsing failed: '{}'", line)));
                }

                match (sphere_position, sphere_radius) {
                    (Some(pos), Some(radius)) => {
                        scene.spheres.push(Sphere::new(pos, radius));
                        sphere_position = None;
                        sphere_radius = None;
                        is_parsing_sphere = false;
                    }
                    _ => { /* do nothing */ }
                }
            } else {
                if line_trimmed.starts_with("sphere:") {
                    is_parsing_sphere = true;
                } else {
                    return Result::Err(
                        io::Error::new(
                            io::ErrorKind::Other,
                            format!("parsing failed: '{}'", line)));
                }
            }
        }

        Result::Ok(scene)
    }
}
