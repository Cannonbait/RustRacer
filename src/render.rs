use super::model::types::*;
use super::*;

pub fn render(objects: Vec<Box<dyn Shape>>, options: Options) {
    let mut window = Window::new(
        options.window_title.as_str(),
        options.width,
        options.height,
        WindowOptions::default(),
    )
    .unwrap();

    let camera = Camera {
        pos: Vector3f {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
    };
    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let rays = generate_rays(&camera, &options);
        let trace = trace(&rays, &objects);

        let buffer: Vec<_> = trace
            .iter()
            .map(|opt| opt.map_or(options.background, |c| c.to_u32()))
            .collect();

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, options.width, options.height)
            .unwrap();
        // break;
    }
}

fn trace(rays: &Vec<Ray>, shapes: &Vec<Box<dyn Shape>>) -> Vec<Option<Color>> {
    return rays.iter().map(|ray| ray.intersect(&shapes)).collect();
}

fn generate_rays(camera: &Camera, options: &Options) -> Vec<Ray> {
    let width = options.width as Fu;
    let height = options.height as Fu;
    let aspect_ratio = width / height;
    let fov = (options.fov as Fu / 2.0).tanh();
    let mut rays = Vec::<Ray>::with_capacity(options.height * options.width);

    for j in 0..options.height {
        for i in 0..options.width {
            let ndc_x = (i as Fu + 0.5) / width;
            let ndc_y = (j as Fu + 0.5) / height;
            let screen_x = (ndc_x * 2.0 - 1.0) * aspect_ratio * fov;
            let screen_y = (1.0 - ndc_y * 2.0) * fov;
            rays.push(Ray::new(
                camera.pos,
                Vector3f {
                    x: screen_x,
                    y: screen_y,
                    z: -1.0,
                },
            ));
        }
    }
    return rays;
}
