extern crate bmp;


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
