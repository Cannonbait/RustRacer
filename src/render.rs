use super::*;

pub fn render(objects: Vec<Box<dyn Shape>>, lights: Vec<Light>, options: Options) {
    let mut window = Window::new(
        options.window_title.as_str(),
        options.width,
        options.height,
        WindowOptions::default(),
    )
    .unwrap();

    let mut camera = Camera {
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
        let buffer: Vec<Cu> = trace(&rays, &options, &objects);
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, options.width, options.height)
            .unwrap();
        // break;
    }
}

fn trace(rays: &Vec<Ray>, options: &Options, shapes: &Vec<Box<dyn Shape>>) -> Vec<Cu> {
    return rays
        .iter()
        .map(|ray| ray.intersect(&shapes, options.background))
        .collect();
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
            rays.push(Ray {
                pos: camera.pos,
                dir: Vector3f {
                    x: screen_x,
                    y: screen_y,
                    z: -1.0,
                },
            });
        }
    }
    return rays;
}
