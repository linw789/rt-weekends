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
        let mut sphere_size: Option<Fp> = None;

        for line in BufReader::new(file).lines() {
            let line = line.unwrap();
            let line_trimmed = line.trim_start();
            if is_parsing_sphere {
                if line_trimmed.starts_with("position") {
                    let mut pos_components = line_trimmed.trim_start_matches("position:").split(",");
                    sphere_position = Some([
                        pos_components.next().unwrap().trim().parse::<Fp>().unwrap(),
                        pos_components.next().unwrap().trim().parse::<Fp>().unwrap(),
                        pos_components.next().unwrap().trim().parse::<Fp>().unwrap()]);
                } else if line_trimmed.starts_with("size") {
                    let size = line_trimmed.trim_start_matches("size:");
                    sphere_size = Some(size.trim().parse::<Fp>().unwrap());
                } else {
                    return Result::Err(
                        io::Error::new(
                            io::ErrorKind::Other,
                            format!("parsing failed: unexpected token")));
                }

                match (sphere_position, sphere_size) {
                    (Some(pos), Some(size)) => {
                        scene.spheres.push(Sphere::new(pos, size));
                        sphere_position = None;
                        sphere_size = None;
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
                            format!("parsing failed: unexpected token")));
                }
            }
        }

        Result::Ok(scene)
    }
}
