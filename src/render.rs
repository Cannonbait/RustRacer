use super::*;

pub fn render(objects: Vec<Box<dyn Intersectable>>, lights: Vec<Light>, options: Options) {
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
        window.get_keys().map(|keys| {
            for t in keys {
                match t {
                    Key::W => {
                        camera.pos.y += 1.0;
                    }
                    Key::S => {
                        camera.pos.y -= 1.0;
                    }
                    _ => (),
                }
            }
        });
        println!("{}", camera.pos.y);

        let rays = generate_rays(&camera, &options);
        let buffer: Vec<u32> = intersect(&rays, &options, &objects);
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, options.width, options.height)
            .unwrap();
    }
}

fn intersect(
    rays: &Vec<Ray>,
    options: &Options,
    objects: &Vec<Box<dyn Intersectable>>,
) -> Vec<u32> {
    return rays
        .iter()
        .map(|ray| ray.intersect(&objects).unwrap_or(options.background))
        .collect();
}

fn generate_rays(camera: &Camera, options: &Options) -> Vec<Ray> {
    let width = options.width as FloatingUnit;
    let height = options.height as FloatingUnit;
    let aspect_ratio = width / height;
    let fov = (options.fov as FloatingUnit / 2.0).tanh();
    let mut rays = Vec::<Ray>::with_capacity(options.height * options.width);

    for j in 0..options.height {
        for i in 0..options.width {
            let ndc_x = (i as FloatingUnit + 0.5) / width;
            let ndc_y = (j as FloatingUnit + 0.5) / height;
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
