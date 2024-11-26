use std::thread::sleep;
use std::time::Duration;

const WIDTH: usize = 160;
const HEIGHT: usize = 44;
const DISTANCE_FROM_CAM: f32 = 100.0;
const K1: f32 = 40.0;
const BACKGROUND_ASCII: char = '.';

fn calculate_x(i: f32, j: f32, k: f32, a: f32, b: f32, c: f32) -> f32 {
    j * a.sin() * b.sin() * c.cos() - k * a.cos() * b.sin() * c.cos()
        + j * a.cos() * c.sin()
        + k * a.sin() * c.sin()
        + i * b.cos() * c.cos()
}

fn calculate_y(i: f32, j: f32, k: f32, a: f32, b: f32, c: f32) -> f32 {
    j * a.cos() * c.cos() + k * a.sin() * c.cos() - j * a.sin() * b.sin() * c.sin()
        + k * a.cos() * b.sin() * c.sin()
        - i * b.cos() * c.sin()
}

fn calculate_z(i: f32, j: f32, k: f32, a: f32, b: f32) -> f32 {
    k * a.cos() * b.cos() - j * a.sin() * b.cos() + i * b.sin()
}

fn calculate_for_surface(
    cube_x: f32,
    cube_y: f32,
    cube_z: f32,
    ch: char,
    buffer: &mut Vec<char>,
    z_buffer: &mut Vec<f32>,
    a: f32,
    b: f32,
    c: f32,
    horizontal_offset: f32,
) {
    let x = calculate_x(cube_x, cube_y, cube_z, a, b, c);
    let y = calculate_y(cube_x, cube_y, cube_z, a, b, c);
    let z = calculate_z(cube_x, cube_y, cube_z, a, b) + DISTANCE_FROM_CAM;

    let ooz = 1.0 / z;
    let xp = (WIDTH as f32 / 2.0 + horizontal_offset + K1 * ooz * x * 2.0) as isize;
    let yp = (HEIGHT as f32 / 2.0 + K1 * ooz * y) as isize;

    if xp >= 0 && xp < WIDTH as isize && yp >= 0 && yp < HEIGHT as isize {
        let idx = xp as usize + (yp as usize) * WIDTH;
        if ooz > z_buffer[idx] {
            z_buffer[idx] = ooz;
            buffer[idx] = ch;
        }
    }
}

fn main() {
    let mut buffer = vec![BACKGROUND_ASCII; WIDTH * HEIGHT];
    let mut z_buffer = vec![0.0; WIDTH * HEIGHT];

    let mut a = 0.0;
    let mut b = 0.0;
    let mut c = 0.0;

    println!("\x1b[2J"); // Clear screen

    loop {
        buffer.fill(BACKGROUND_ASCII);
        z_buffer.fill(0.0);

        let cube_width = 20.0;
        let horizontal_offset = -2.0 * cube_width;

        for cube_x in (-cube_width as isize..=cube_width as isize).map(|x| x as f32) {
            for cube_y in (-cube_width as isize..=cube_width as isize).map(|y| y as f32) {
                calculate_for_surface(
                    cube_x,
                    cube_y,
                    -cube_width,
                    '@',
                    &mut buffer,
                    &mut z_buffer,
                    a,
                    b,
                    c,
                    horizontal_offset,
                );
                calculate_for_surface(
                    cube_width,
                    cube_y,
                    cube_x,
                    '$',
                    &mut buffer,
                    &mut z_buffer,
                    a,
                    b,
                    c,
                    horizontal_offset,
                );
                calculate_for_surface(
                    -cube_width,
                    cube_y,
                    -cube_x,
                    '~',
                    &mut buffer,
                    &mut z_buffer,
                    a,
                    b,
                    c,
                    horizontal_offset,
                );
                calculate_for_surface(
                    -cube_x,
                    cube_y,
                    cube_width,
                    '#',
                    &mut buffer,
                    &mut z_buffer,
                    a,
                    b,
                    c,
                    horizontal_offset,
                );
                calculate_for_surface(
                    cube_x,
                    -cube_width,
                    -cube_y,
                    ';',
                    &mut buffer,
                    &mut z_buffer,
                    a,
                    b,
                    c,
                    horizontal_offset,
                );
                calculate_for_surface(
                    cube_x,
                    cube_width,
                    cube_y,
                    '+',
                    &mut buffer,
                    &mut z_buffer,
                    a,
                    b,
                    c,
                    horizontal_offset,
                );
            }
        }

        print!("\x1b[H"); // Move cursor to top left
        for (i, ch) in buffer.iter().enumerate() {
            if i % WIDTH == 0 && i != 0 {
                println!();
            }
            print!("{}", ch);
        }

        a += 0.05;
        b += 0.05;
        c += 0.01;

        sleep(Duration::from_millis(16));
    }
}
