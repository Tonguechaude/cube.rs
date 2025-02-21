use libm::*;

struct Rotation {
    a : f32,
    b : f32,
    c : f32,
}

struct Camera {
    cube_width : f32,
    width : i32,
    height : i32,
    z_buffer : Vec<f32>,
    buffer : Vec<char>,
    background_ascii_code : i32,
    distance_from_cam : i32,
    horizontal_offset : f32,
    k1 : f32,
    increment_speed : f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            cube_width : 20f32,
            width : 160,
            height : 44,
            z_buffer: vec![Self.width as f32 * Self.height as f32],
            buffer: vec![' '; Self.width as usize * Self.height as usize],
            background_ascii_code: '.' as i32, // get ascii code of '.'
            distance_from_cam: 100,
            horizontal_offset: None as f32,
            k1: 40f32,
            increment_speed: 0.6,
        }
    }
}

struct Cube {
    x : f32,
    y : f32,
    z : f32,
    ooz : f32,
    xp : i32,
    yp : i32,
    idx : i32
}

fn main () -> () {

}