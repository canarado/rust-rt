use crate::util::clampf;

pub type Color = (f64, f64, f64);

pub fn write_color_to_stdout(pixel_color: Color, samples_per_pixel: u64) {

    let mut r = pixel_color.0;
    let mut g = pixel_color.1;
    let mut b = pixel_color.2;

    let scale: f64 = 1.0 / samples_per_pixel as f64;

    // r = (r * scale);
    // g = (g * scale);
    // b = (b * scale);
    r *= scale;
    g *= scale;
    b *= scale;

    println!("{} {} {}", (256.0 * clampf(r, 0.0, 0.999)) as u64, (256.0 * clampf(g, 0.0, 0.999)) as u64, (256.0 * clampf(b, 0.0, 0.999)) as u64)
}

pub fn write_ppm_header_to_stdout(image_width: u64, image_height: u64) {
    println!("P3\n{} {}\n255", image_width, image_height);
}