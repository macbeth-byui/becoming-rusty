// use std::time::{SystemTime, UNIX_EPOCH};
use raylib::prelude::*;
use std::thread;

const FRACTAL_ITERATIONS: i32 = 150;
const FRACTAL_ESCAPE: f64 = 2.0;
const WINDOW_HEIGHT: i32 = 800;
const WINDOW_WIDTH: i32 = 800;
const INIT_VIRTUAL_GRID_XMIN: f64 = -2.0;
const INIT_VIRTUAL_GRID_XMAX: f64 = 2.0;
const INIT_VIRTUAL_GRID_YMIN: f64 = -2.0;
const INIT_VIRTUAL_GRID_YMAX: f64 = 2.0;

struct Mandelbrot {
    xmin: f64,
    xmax: f64,
    ymin: f64,
    ymax: f64,
}

struct Point {
    x: i32,
    y: i32,
    color: Color,
}

struct Worker {
    x: f64,
    delta_y: f64,
    xmin: f64,
    xmax: f64,
    ymin: f64,
    ymax: f64,
}

impl Mandelbrot {
    pub fn new() -> Self {
        Mandelbrot {
            xmin: INIT_VIRTUAL_GRID_XMIN,
            xmax: INIT_VIRTUAL_GRID_XMAX,
            ymin: INIT_VIRTUAL_GRID_YMIN,
            ymax: INIT_VIRTUAL_GRID_YMAX,
        }
    }

    pub fn reset(&mut self) {
        self.xmin = INIT_VIRTUAL_GRID_XMIN;
        self.xmax = INIT_VIRTUAL_GRID_XMAX;
        self.ymin = INIT_VIRTUAL_GRID_YMIN;
        self.ymax = INIT_VIRTUAL_GRID_YMAX;
    }

    // From ChatGPT
    fn interpolate_color(color1: (u8, u8, u8), color2: (u8, u8, u8), factor: f64) -> (u8, u8, u8) {
        let r = (color1.0 as f64 * (1.0 - factor) + color2.0 as f64 * factor) as u8;
        let g = (color1.1 as f64 * (1.0 - factor) + color2.1 as f64 * factor) as u8;
        let b = (color1.2 as f64 * (1.0 - factor) + color2.2 as f64 * factor) as u8;
        (r, g, b)
    }

    fn generate_color(n: i32) -> Color {
        // let colors = [  // Sunset
        //     (139, 69, 19),   // Dark brown
        //     (178, 34, 34),   // Firebrick
        //     (205, 92, 92),   // Indian red
        //     (210, 105, 30),  // Chocolate
        //     (244, 164, 96),  // Sandy brown
        //     (160, 82, 45),   // Sienna
        // ];
        let colors = [
            // Rainbow
            (255, 0, 0),   // Red
            (255, 165, 0), // Orange
            (255, 255, 0), // Yellow
            (0, 255, 0),   // Green
            (0, 0, 255),   // Blue
            (75, 0, 130),  // Indigo
            (148, 0, 211), // Violet
        ];
        // let colors = [ // Cosmic
        //     (0, 0, 0),             // Deep Space Black
        //     (102, 0, 102),         // Cosmic Purple
        //     (0, 0, 255),           // Electric Blue
        //     (255, 105, 180),       // Galactic Pink
        // ];
        let steps = (FRACTAL_ITERATIONS as f64) / (colors.len() - 1) as f64;
        let segment = (n as f64) / steps;
        let index = segment.floor() as usize;
        let color1 = colors[index];
        let color2 = colors[index + 1];
        let factor = segment.fract();
        let (r, g, b) = Mandelbrot::interpolate_color(color1, color2, factor);
        Color { r, g, b, a: 255 }
    }

    fn calc_mandelbrot_point(x: f64, y: f64) -> Color {
        let mut prev_x: f64 = x;
        let mut prev_y: f64 = y;
        let mut curr_x: f64;
        let mut curr_y: f64;
        let mut escape_count: i32 = -1;
        for count in 0..FRACTAL_ITERATIONS {
            curr_x = (prev_x * prev_x) - (prev_y * prev_y) + x;
            curr_y = (2.0 * (prev_x * prev_y)) + y;
            prev_x = curr_x;
            prev_y = curr_y;
            if (curr_x * curr_x + curr_y * curr_y).sqrt() > FRACTAL_ESCAPE {
                escape_count = count;
                break;
            }
        }
        if escape_count == -1 {
            return Color::BLACK;
        }
        Mandelbrot::generate_color(escape_count)
    }

    fn draw_mandelbrot_worker(worker: Worker) -> Vec<Point> {
        let mut results = Vec::<Point>::new();
        let mut y = worker.ymin;
        while y <= worker.ymax {
            let color = Mandelbrot::calc_mandelbrot_point(worker.x, y);
            let x_int = ((worker.x - worker.xmin) / (worker.xmax - worker.xmin)
                * WINDOW_WIDTH as f64) as i32;
            let y_int =
                ((y - worker.ymin) / (worker.ymax - worker.ymin) * WINDOW_HEIGHT as f64) as i32;
            results.push(Point {
                x: x_int,
                y: y_int,
                color,
            });
            y += worker.delta_y;
        }
        results
    }

    fn draw_mandelbrot(&self, image: &mut Image) {
        let mut threads = Vec::<thread::JoinHandle<Vec<Point>>>::new();
        let mut x = self.xmin;
        let delta_x: f64 = (self.xmax - self.xmin) / WINDOW_WIDTH as f64;
        let delta_y: f64 = (self.ymax - self.ymin) / WINDOW_HEIGHT as f64;
        while x <= self.xmax {
            let worker = Worker {
                x,
                delta_y,
                xmin: self.xmin,
                xmax: self.xmax,
                ymin: self.ymin,
                ymax: self.ymax,
            };
            let worker_thread = thread::spawn(|| Mandelbrot::draw_mandelbrot_worker(worker));
            threads.push(worker_thread);
            x += delta_x;
        }
        for thread_handle in threads {
            let result = thread_handle.join().unwrap();
            for point in result {
                image.draw_pixel(point.x, point.y, point.color);
            }
        }
    }

    fn zoom(&mut self, ratio: f64) {
        let delta_x: f64 = (self.xmax - self.xmin) / WINDOW_WIDTH as f64;
        let delta_y: f64 = (self.ymax - self.ymin) / WINDOW_HEIGHT as f64;
        self.xmin += ratio * delta_x;
        self.xmax -= ratio * delta_x;
        self.ymin += ratio * delta_y;
        self.ymax -= ratio * delta_y;
    }

    fn delta_x(&mut self, delta: i32) {
        let delta_x: f64 = (self.xmax - self.xmin) / WINDOW_WIDTH as f64;
        self.xmin += delta_x * delta as f64;
        self.xmax += delta_x * delta as f64;
    }

    fn delta_y(&mut self, delta: i32) {
        let delta_y: f64 = (self.ymax - self.ymin) / WINDOW_HEIGHT as f64;
        self.ymin += delta_y * delta as f64;
        self.ymax += delta_y * delta as f64;
    }
}

fn main() {
    let mut mandelbrot = Mandelbrot::new();

    raylib::logging::set_trace_log(TraceLogLevel::LOG_WARNING);

    let (mut rl, thread) = raylib::init()
        .size(WINDOW_HEIGHT, WINDOW_WIDTH)
        .title("Mandelbrot")
        .build();

    let mut image = Image::gen_image_color(WINDOW_HEIGHT, WINDOW_WIDTH, Color::BLACK);

    // let mut max = 0;
    // let mut min = 999999;
    // let mut count = 0;
    // let mut sum = 0;

    rl.set_target_fps(60);

    let mut redraw = true;

    while !rl.window_should_close() {
        // let time1 = SystemTime::now()
        //     .duration_since(UNIX_EPOCH)
        //     .unwrap()
        //     .as_micros();

        let pressed_key = rl.get_key_pressed();
        let mut zoom = 1.0;
        let mut exit = false;
        if let Some(pressed_key) = pressed_key {
            match pressed_key {
                KeyboardKey::KEY_A => {
                    mandelbrot.zoom(-10.0);
                }
                KeyboardKey::KEY_S => {
                    mandelbrot.zoom(10.0);
                }
                KeyboardKey::KEY_Q => {
                    exit = true;
                }
                KeyboardKey::KEY_R => {
                    mandelbrot.reset();
                }
                KeyboardKey::KEY_UP => {
                    mandelbrot.delta_y(-10);
                }
                KeyboardKey::KEY_DOWN => {
                    mandelbrot.delta_y(10);
                }
                KeyboardKey::KEY_RIGHT => {
                    mandelbrot.delta_x(10);
                }
                KeyboardKey::KEY_LEFT => {
                    mandelbrot.delta_x(-10);
                }
                _ => (),
            }
        }

        if exit {
            break;
        }
        mandelbrot.draw_mandelbrot(&mut image);
        let texture = rl
            .load_texture_from_image(&thread, &image)
            .expect("Error creating texture");
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_texture(&texture, 0, 0, Color::WHITE);

        // let time2 = SystemTime::now()
        // .duration_since(UNIX_EPOCH)
        // .unwrap()
        // .as_micros();

        // let total = time2 - time1;
        // count += 1;
        // sum += total;
        // if total > max {
        //     max = total;
        // }
        // if total < min {
        //     min = total;
        // }
        // println!("min = {} max = {} avg = {}", min, max, sum / count);
    }
}
