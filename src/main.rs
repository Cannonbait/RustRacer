extern crate minifb;

mod camera;
mod circle;
mod light;
mod options;
mod ray;
mod render;
mod shape;
mod sphere;
mod vector;
use camera::*;
use circle::*;
use light::*;
use minifb::*;
use options::*;
use ray::*;
use render::*;
use shape::*;
use sphere::*;
use vector::*;

type Fu = f64;
type Cu = u32;

fn main() {
    let options = Options {
        width: 2560,
        height: 1440,
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

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

fn load_objects() -> Vec<Box<dyn Shape>> {
    let mut vec: Vec<Box<dyn Shape>> = Vec::new();
    vec.push(Box::new(Sphere {
        pos: Vector3f {
            x: 0.0,
            y: 0.0,
            z: -4.2,
        },
        radius: 1.5,
        color: from_u8_rgb(0, 128, 0),
    }));
    // vec.push(Box::new(Circle {
    //     pos: Vector3f {
    //         x: 0.0,
    //         y: 0.0,
    //         z: -3.0,
    //     },
    //     radius: 1.0,
    //     color: from_u8_rgb(0, 0, 155),
    // }));
    return vec;
}

fn load_lights() -> Vec<Light> {
    let vec: Vec<Light> = Vec::new();
    return vec;
}
