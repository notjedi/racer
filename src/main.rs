const IMAGE_WIDTH: usize = 256;
const IMAGE_HEIGHT: usize = 256;

fn gen_ppm_file() {
    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");

    for i in 0..IMAGE_HEIGHT {
        for j in 0..IMAGE_WIDTH {
            let r = j as f32 / (IMAGE_WIDTH - 1) as f32;
            let g = i as f32 / (IMAGE_HEIGHT - 1) as f32;
            let b = 0.25;

            let r_i32 = (r * 255.999) as i32;
            let g_i32 = (g * 255.999) as i32;
            let b_i32 = (b * 255.999) as i32;

            println!("{r_i32} {g_i32} {b_i32}");
        }
    }
}

fn main() {
    gen_ppm_file();
}
