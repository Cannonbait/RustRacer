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
use vector::*;

fn main() {
    let options = Options {
        width: 5,
        height: 5,
        fov: 90,
        max_depth: 5,
        file_name: String::from("render"),
        window_title: String::from("rustRacer"),
    };
    let objects = load_objects();
    let lights = load_lights();

    render(objects, lights, options);
}

fn load_objects() -> Vec<Object> {
    let vec: Vec<Object> = Vec::new();
    return vec;
}

fn load_lights() -> Vec<Light> {
    let vec: Vec<Light> = Vec::new();
    return vec;
}
