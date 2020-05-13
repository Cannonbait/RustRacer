extern crate minifb;

mod camera;
mod light;
mod object;
mod options;
mod ray;
mod render;
mod shape;
mod vector;
use camera::*;
use light::*;
use minifb::*;
use object::*;
use options::*;
use ray::*;
use render::*;
use shape::*;
use vector::*;

fn main() {
    let options = Options {
        width: 640,
        height: 480,
        fov: 90,
        max_depth: 5,
        file_name: String::from("render"),
        window_title: String::from("rustRacer"),
    };
    let objects = load_objects();
    let lights = load_lights();

    render(objects, lights, options);
}

fn load_objects() -> Vec<Box<dyn Intersectable>> {
    let mut vec: Vec<Box<dyn Intersectable>> = Vec::new();
    vec.push(Box::new(Circle {
        pos: Vector3f {
            x: 2.0,
            y: 0.0,
            z: 2.0,
        },
        radius: 1.0,
        colour: 60000,
    }));
    return vec;
}

fn load_lights() -> Vec<Light> {
    let vec: Vec<Light> = Vec::new();
    return vec;
}
