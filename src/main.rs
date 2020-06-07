extern crate minifb;

mod model;
mod options;
mod render;
mod shapes;

use minifb::*;
use model::camera::*;
use model::color::*;
use model::ray::*;
use model::shape::*;
use model::vector::*;
use options::*;
use render::*;
use shapes::plane::*;
use shapes::sphere::*;

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
            z: -3.0,
        },
        radius: 1.0,
        color: Color {
            r: 242,
            g: 119,
            b: 119,
        },
    }));
    vec.push(Box::new(Plane::new(
        Vector3f {
            x: -10.0,
            y: 0.0,
            z: 0.0,
        },
        Vector3f {
            x: -1.0,
            y: 0.0,
            z: 0.0,
        },
        Color {
            r: 200,
            g: 200,
            b: 200,
        },
    )));
    vec.push(Box::new(Plane::new(
        Vector3f {
            x: 10.0,
            y: 0.0,
            z: 0.0,
        },
        Vector3f {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
        Color {
            r: 200,
            g: 200,
            b: 200,
        },
    )));
    vec.push(Box::new(Plane::new(
        Vector3f {
            x: 0.0,
            y: 0.0,
            z: -10.0,
        },
        Vector3f {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
        Color {
            r: 200,
            g: 200,
            b: 200,
        },
    )));
    vec.push(Box::new(Plane::new(
        Vector3f {
            x: 0.0,
            y: 5.0,
            z: 0.0,
        },
        Vector3f {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        Color {
            r: 200,
            g: 200,
            b: 200,
        },
    )));
    vec.push(Box::new(Plane::new(
        Vector3f {
            x: 0.0,
            y: -5.0,
            z: 0.0,
        },
        Vector3f {
            x: 0.0,
            y: -1.0,
            z: 0.0,
        },
        Color {
            r: 200,
            g: 200,
            b: 200,
        },
    )));

    return vec;
}

// fn load_lights() -> Vec<Light> {
//     let vec: Vec<Light> = Vec::new();
//     return vec;
// }
