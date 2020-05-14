extern crate minifb;

mod camera;
mod light;
mod object;
mod options;
mod ray;
mod render;
mod shape;
mod sphere;
mod vector;
use camera::*;
use light::*;
use minifb::*;
use object::*;
use options::*;
use ray::*;
use render::*;
use shape::*;
use sphere::*;
use vector::*;

type FloatingUnit = f64;

fn main() {
    let options = Options {
        width: 640,
        height: 480,
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

fn load_objects() -> Vec<Box<dyn Intersectable>> {
    let mut vec: Vec<Box<dyn Intersectable>> = Vec::new();
    vec.push(Box::new(Sphere {
        pos: Vector3f {
            x: 0.0,
            y: 0.0,
            z: -2.0,
        },
        radius: 1.0,
        colour: from_u8_rgb(250, 128, 114),
    }));
    return vec;
}

fn load_lights() -> Vec<Light> {
    let vec: Vec<Light> = Vec::new();
    return vec;
}
