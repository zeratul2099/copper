extern crate image;

use image::{ImageBuffer,Rgba};

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

pub fn lsb_embed(filename: String, message: String) {
    let mut img = image::open(filename).unwrap().to_rgba();
    let (width, _height) = img.dimensions();
    let mut msg_len = message.len();
    // embed msg len first
    for i in 0..32 as u32 {
        let x: u32 = i % width;
        let y: u32 = i / width;
        let bit_to_embed = msg_len % 2;
        msg_len = msg_len / 2;
        embed_bit_into_pixel(&mut img, x, y,  bit_to_embed);

    }
    // iterate chars
    for (byte_idx, ch) in message.bytes().enumerate() {
        println!("{}", ch);
        let mut byte: u8 = ch as u8;
        // iterate bits
        for i in 0..8 as u32 {
            let bit_idx = byte_idx as u32 + i + 32;
            let x: u32 = bit_idx % width;
            let y: u32 = bit_idx / width;
            let bit_to_embed = byte % 2;
            byte = byte / 2;
            println!("  {}, {}", bit_to_embed, byte);
            // only embed in red
            embed_bit_into_pixel(&mut img, x, y,  bit_to_embed as usize);
        }
    }
    println!("+++{}+++", message.len());
    img.save("test.png").unwrap();
}

pub fn lsb_extract(filename: String) {
    let img = image::open(filename).unwrap().to_rgba();
    let (width, _height) = img.dimensions();
    // extract message length
    let mut msg_len: u32 = 0;
    for i in (0..32).rev() {
        let x: u32 = i % width;
        let y: u32 = i / width;
        let bit = extract_bit_from_pixel(&img, x, y);
        msg_len = msg_len << 1;
        msg_len += bit as u32;
    }
    println!("---{}---", msg_len);
    // extract char by char
    for char_idx in (0..msg_len) {
        let char: u8 = 0;
        for i in (0..8).rev() {
            let bit_idx: u32 = char_idx * 8 + i;
        }

    }
}

fn embed_bit_into_pixel(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, x: u32, y: u32, bit_to_embed: usize) {//-> &mut ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut pixel = img.get_pixel(x, y).clone();
    // only embed in red
    pixel.data[0] = embed_bit_into_byte(pixel.data[0], bit_to_embed);
    img.put_pixel(x, y, pixel);
    //img
}

fn embed_bit_into_byte(mut byte: u8, bit_to_embed: usize) -> u8 {
    if bit_to_embed == 0 && byte % 2 == 1 {
        byte -= 1;
    } else if bit_to_embed == 1 && byte % 2 == 0 {
        byte += 1;
    }
    byte
}

fn extract_bit_from_pixel(img: &ImageBuffer<Rgba<u8>, Vec<u8>>, x: u32, y: u32) -> u8 {
    let pixel = img.get_pixel(x, y);
    pixel.data[0] % 2
    
}

