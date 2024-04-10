extern crate image;

use std::error::Error;
use std::fs::File;
use std::io::Write;

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
        for i in 0..NX {
            let r = i as f64 / (NX as f64 - 1.0);
            let g = j as f64 / (NY as f64 - 1.0);
            let b = 0.0;

            let ir = (COLOR * r) as u8;
            let ig = (COLOR * g) as u8;
            let ib = (COLOR * b) as u8;

            // Write to PPM file
            writeln!(ppm_file, "{} {} {}", ir, ig, ib)?;

            // Write to PNG image
            png_image.put_pixel(i, j, image::Rgb([ir, ig, ib]));
        }
    }

    // Save PNG image
    png_image.save("out.png")?;

    Ok(())
}

fn write_ppm_header(file: &mut File) -> Result<(), Box<dyn Error>> {
    writeln!(file, "P3")?;
    writeln!(file, "{} {}", NX, NY)?;
    writeln!(file, "255")?;
    Ok(())
}
