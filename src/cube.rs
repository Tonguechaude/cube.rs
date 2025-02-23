use std::{thread::sleep, time::Duration};

const WIDTH: usize = 160;
const HEIGHT: usize = 44;
const DISTANCE_FROM_CAM: f32 = 100.0;
const K1: f32 = 40.0;
const INCREMENT_SPEED: f32 = 0.6;
const BACKGROUND_CHAR: char = ' ';

struct Rotation {
    a: f32,
    b: f32,
    c: f32,
}

impl Rotation {
    fn new() -> Self {
        Rotation {
            a: 0.0,
            b: 0.0,
            c: 0.0,
        }
    }

    fn calculate_x(&self, i: f32, j: f32, k: f32) -> f32 {
        j * self.a.sin() * self.b.sin() * self.c.cos()
            - k * self.a.cos() * self.b.sin() * self.c.cos()
            + j * self.a.cos() * self.c.sin()
            + k * self.a.sin() * self.c.sin()
            + i * self.b.cos() * self.c.cos()
    }

    fn calculate_y(&self, i: f32, j: f32, k: f32) -> f32 {
        j * self.a.cos() * self.c.cos() + k * self.a.sin() * self.c.cos()
            - j * self.a.sin() * self.b.sin() * self.c.sin()
            + k * self.a.cos() * self.b.sin() * self.c.sin()
            - i * self.b.cos() * self.c.sin()
    }

    fn calculate_z(&self, i: f32, j: f32, k: f32) -> f32 {
        k * self.a.cos() * self.b.cos() - j * self.a.sin() * self.b.cos() + i * self.b.sin()
    }
}

#[allow(clippy::too_many_arguments)]
fn calculate_for_surface(
    rotation: &Rotation,
    cube_x: f32,
    cube_y: f32,
    cube_z: f32,
    ch: char,
    buffer: &mut [char],
    z_buffer: &mut [f32],
    horizontal_offset: f32,
) {
    let x = rotation.calculate_x(cube_x, cube_y, cube_z);
    let y = rotation.calculate_y(cube_x, cube_y, cube_z);
    let z = rotation.calculate_z(cube_x, cube_y, cube_z) + DISTANCE_FROM_CAM;

    let ooz = 1.0 / z;
    let xp = (WIDTH as f32 / 2.0 + horizontal_offset + K1 * ooz * x * 2.0) as isize;
    let yp = (HEIGHT as f32 / 2.0 + K1 * ooz * y) as isize;

    let idx = (xp + yp * WIDTH as isize) as usize;
    if idx < WIDTH * HEIGHT && ooz > z_buffer[idx] {
            z_buffer[idx] = ooz;
            buffer[idx] = ch;
    }
}

fn main() {
    let mut rotation = Rotation::new();
    println!("\x1b[2J");

    loop {
        let mut buffer = vec![BACKGROUND_CHAR; WIDTH * HEIGHT];
        let mut z_buffer = vec![0.0; WIDTH * HEIGHT];

        let mut draw_cube = |cube_width: f32, horizontal_offset: f32| {
            for cube_x in (-cube_width as i32)..(cube_width as i32) {
                for cube_y in (-cube_width as i32)..(cube_width as i32) {
                    let cube_x = cube_x as f32 + INCREMENT_SPEED;
                    let cube_y = cube_y as f32 + INCREMENT_SPEED;

                    calculate_for_surface(
                        &rotation,
                        cube_x,
                        cube_y,
                        -cube_width,
                        '@',
                        &mut buffer,
                        &mut z_buffer,
                        horizontal_offset,
                    );
                    calculate_for_surface(
                        &rotation,
                        cube_width,
                        cube_y,
                        cube_x,
                        '$',
                        &mut buffer,
                        &mut z_buffer,
                        horizontal_offset,
                    );
                    calculate_for_surface(
                        &rotation,
                        -cube_width,
                        cube_y,
                        -cube_x,
                        '~',
                        &mut buffer,
                        &mut z_buffer,
                        horizontal_offset,
                    );
                    calculate_for_surface(
                        &rotation,
                        -cube_x,
                        cube_y,
                        cube_width,
                        '#',
                        &mut buffer,
                        &mut z_buffer,
                        horizontal_offset,
                    );
                    calculate_for_surface(
                        &rotation,
                        cube_x,
                        -cube_width,
                        -cube_y,
                        ';',
                        &mut buffer,
                        &mut z_buffer,
                        horizontal_offset,
                    );
                    calculate_for_surface(
                        &rotation,
                        cube_x,
                        cube_width,
                        cube_y,
                        '+',
                        &mut buffer,
                        &mut z_buffer,
                        horizontal_offset,
                    );
                }
            }
        };

        draw_cube(20.0, -40.0);
        // draw_cube(10.0, 10.0);
        // draw_cube(5.0, 40.0);

        print!("\x1b[H");
        for (i, c) in buffer.iter().enumerate() {
            if i % WIDTH == 0 {
                println!();
            }
            print!("{}", c);
        }

        rotation.a += 0.05;
        rotation.b += 0.05;
        rotation.c += 0.01;

        sleep(Duration::from_millis(16));
    }
}
