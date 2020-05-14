extern crate minifb;

mod camera;
mod circle;
mod color;
mod light;
mod options;
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
        background: 0,
    };
    let objects = load_objects();
    let lights = load_lights();

    render(objects, lights, options);
}

fn load_objects() -> Vec<Box<dyn Shape>> {
    let mut vec: Vec<Box<dyn Shape>> = Vec::new();
    vec.push(Box::new(Sphere {
        pos: Vector3f {
            x: 1.0,
            y: 1.0,
            z: -5.0,
        },
        radius: 3.0,
        color: Color { r: 0, g: 128, b: 0 },
    }));
    // vec.push(Box::new(Sphere {
    //     pos: Vector3f {
    //         x: 1.0,
    //         y: 0.0,
    //         z: -3.0,
    //     },
    //     radius: 1.0,
    //     color: Color { r: 0, g: 0, b: 128 },
    // }));
    return vec;
}

fn load_lights() -> Vec<Light> {
    let vec: Vec<Light> = Vec::new();
    return vec;
}
