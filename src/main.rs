mod vector;

use vector::Vec3;

const IMAGE_WIDTH: usize = 256;
const IMAGE_HEIGHT: usize = 256;

fn gen_ppm_file() {
    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");

    for i in 0..IMAGE_HEIGHT {
        for j in 0..IMAGE_WIDTH {
            let color = Vec3::new(
                j as f32 / (IMAGE_WIDTH - 1) as f32,
                i as f32 / (IMAGE_HEIGHT - 1) as f32,
                0.25,
            );
            let color_scaled = &color * 255.999 as f32;
            println!(
                "{} {} {}",
                color_scaled.r() as i32,
                color_scaled.g() as i32,
                color_scaled.b() as i32
            );
        }
    }
}

fn main() {
    gen_ppm_file();
}
