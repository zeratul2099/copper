extern crate image;


#[cfg(test)]
mod tests {

    use crate::load_image;

    #[test]
    fn it_works() {
        load_image("avatar.png".to_string());
        assert_eq!(2 + 2, 4);
    }
}


pub fn load_image(filename: String) {
    let mut img = image::open(filename).unwrap().to_rgba();

    println!("dimensions {:?}", img.dimensions());
    for y in 90..110 {
        for x in 0..img.dimensions().0 {
            let mut pixel = img.get_pixel(x, y).clone();
            println!("{:?}", pixel.data);
            pixel.data[0] = 0;
            pixel.data[1] = 0;
            pixel.data[2] = 0;
            img.put_pixel(x, y, pixel);
        }
    }
    img.save("test.png").unwrap();
}

