extern crate minifb;

mod camera;
mod circle;
mod color;
mod light;
mod options;
mod plane;
mod ray;
mod render;
mod shape;
mod sphere;
mod vector;
use camera::*;
use circle::*;
use color::*;
use light::*;
use minifb::*;
use options::*;
use plane::*;
use ray::*;
use render::*;
use shape::*;
use sphere::*;
use vector::*;

type Fu = f64;

fn main() {
    let options = Options {
        width: 1280,
        height: 960,
        fov: 90,
        max_depth: 5,
        file_name: String::from("render"),
        window_title: String::from("rustRacer"),
        background: Color { r: 0, g: 0, b: 0 }.to_u32(),
    };
    let objects = load_objects();
    // let lights = load_lights();

    render(objects, options);
}

fn load_objects() -> Vec<Box<dyn Shape>> {
    let mut vec: Vec<Box<dyn Shape>> = Vec::new();
    vec.push(Box::new(Sphere {
        pos: Vector3f {
            x: 0.0,
            y: 0.0,
            z: -20.0,
        },
        radius: 5.0,
        color: Color {
            r: 242,
            g: 119,
            b: 119,
        },
    }));
    vec.push(Box::new(Plane::new(
        Vector3f {
            x: 0.0,
            y: 0.0,
            z: -25.0,
        },
        Vector3f {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        Color {
            r: 24,
            g: 41,
            b: 140,
        },
    )));

    return vec;
}

// fn load_lights() -> Vec<Light> {
//     let vec: Vec<Light> = Vec::new();
//     return vec;
// }
