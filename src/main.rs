extern crate image;

use std::error::Error;
use std::fs::File;
use std::io::Write;

use crate::vec3::vec3::Color;

mod vec3;
use crate::vec3::vec3::write_color;
use crate::vec3::vec3::write_color_png;
mod ray;

const NX: u32 = 256;
const NY: u32 = 256;
const COLOR: f64 = 255.999;

fn main() -> Result<(), Box<dyn Error>> {
    // Create PPM image
    let mut ppm_file = File::create("out.ppm")?;
    write_ppm_header(&mut ppm_file)?;

    // Create PNG image
    let mut png_image = image::RgbImage::new(NX, NY);

    for j in 0..NY {
        let tmp = NY - j;
        println!("Lines remaining: {tmp}");
        for i in 0..NX {
            let pixel_color =
                Color::new(i as f64 / (NX - 1) as f64, j as f64 / (NY - 1) as f64, 0.0);

            write_color(&mut ppm_file, &pixel_color)?;
            write_color_png(&mut png_image, &pixel_color, i, j)?;
        }
    }

    println!("Done");
    // Save PNG image
    png_image.save("newpng.png")?;

    Ok(())
}

fn write_ppm_header(file: &mut File) -> Result<(), Box<dyn Error>> {
    writeln!(file, "P3")?;
    writeln!(file, "{} {}", NX, NY)?;
    writeln!(file, "255")?;
    Ok(())
}
