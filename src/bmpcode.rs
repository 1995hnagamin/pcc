extern crate bmp;

use std::result;


type ResultCodel = result::Result<bmp::Pixel, (u32, u32)>;

fn get_codel(img: &bmp::Image, x: u32, y: u32, codel_size: u32) -> ResultCodel {
    let base = img.get_pixel(x * codel_size, y * codel_size);
    for x in x..(x + codel_size) {
        for y in y..(y + codel_size) {
            let pixel = img.get_pixel(x, y);
            if base != pixel {
                return Err((x, y));
            }
        }
    }
    Ok(base)
}

fn make_matrix(img: &bmp::Image, codel_size: u32) {
    let (width, height) = (img.get_width(), img.get_height());
    if (width % codel_size != 0) | (height % codel_size != 0) {
        panic!("Bad codel size: {} ({}x{})", codel_size, width, height);
    };
    let (wpixels, hpixels) = (width / codel_size, height / codel_size);
    let matrix: Vec<Vec<bmp::Pixel>> = (0..wpixels)
        .map(|x| {
            (0..hpixels)
                .map(|y| match get_codel(img, codel_size, x, y) {
                         Ok(result) => result,
                         Err((bx, by)) => {
                    panic!("Bad pixel: {}x{}", bx, by);
                }
                     })
                .collect()
        })
        .collect();
}

pub fn read_code(file: &str, codel_size: u32) {
    assert!(codel_size > 0);
    let img = bmp::open(file).unwrap_or_else(|e| {
                                                 panic!("Failed to open bmp file: {}", e);
                                             });
    let (width, height) = (img.get_width(), img.get_height());
    if (width % codel_size != 0) | (height % codel_size != 0) {
        panic!("Bad codel size: {} ({}x{})", codel_size, width, height);
    };
}
