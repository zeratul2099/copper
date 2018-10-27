extern crate image;

use image::{ImageBuffer,Rgba};

#[cfg(test)]
mod tests {

    use crate::{lsb_embed, lsb_extract};

    #[test]
    fn it_works() {
        let test_message = "My hovercraft is full of eels!".to_string();
        let cover = image::open("avatar.png".to_string()).unwrap().to_rgba();
        let output = lsb_embed(&cover, &test_message);
        output.save("test.png").unwrap();
        let extracted_message = lsb_extract(&output);
        assert_eq!(test_message, extracted_message);
    }
}


pub fn lsb_embed(cover: &ImageBuffer<Rgba<u8>, Vec<u8>>, message: &String) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut output = cover.clone();
    let (width, _height) = output.dimensions();
    let mut msg_len = message.len();
    // embed msg len first
    for i in 0..32 as u32 {
        let x: u32 = i % width;
        let y: u32 = i / width;
        let bit_to_embed = msg_len % 2;
        msg_len = msg_len / 2;
        embed_bit_into_pixel(&mut output, x, y,  bit_to_embed);

    }
    // iterate chars
    for (byte_idx, ch) in message.bytes().enumerate() {
        //println!("{}", ch);
        let mut byte: u8 = ch as u8;
        // iterate bits
        for i in 0..8 as u32 {
            let bit_idx = byte_idx as u32 * 8 + i + 32;
            let x: u32 = bit_idx % width;
            let y: u32 = bit_idx / width;
            let bit_to_embed = byte % 2;
            byte = byte / 2;
            //println!("  {}, x:{}, y:{}", bit_to_embed, x, y);
            // only embed in red
            embed_bit_into_pixel(&mut output, x, y,  bit_to_embed as usize);
        }
    }
    println!("+++{}+++", message.len());
    output
}

pub fn lsb_extract(steganogram: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> String {
    let (width, _height) = steganogram.dimensions();
    // extract message length
    let mut msg_len: u32 = 0;
    for i in (0..32).rev() {
        let x: u32 = i % width;
        let y: u32 = i / width;
        let bit = extract_bit_from_pixel(&steganogram, x, y);
        msg_len = msg_len << 1;
        msg_len += bit as u32;
    }
    println!("---{}---", msg_len);
    // extract char by char
    let mut message: String = String::new();
    for char_idx in 0..msg_len {
        let mut byte: u8 = 0;
        for i in (0..8).rev() {
            let bit_idx: u32 = char_idx * 8 + i + 32;
            let x: u32 = bit_idx % width;
            let y: u32 = bit_idx / width;
            let bit = extract_bit_from_pixel(&steganogram, x, y);
            //println!("  {}, x:{}, y:{}", bit, x, y);
            byte = byte << 1;
            byte += bit
        }
        message.push(byte as char);
    }
    message
}

fn embed_bit_into_pixel(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, x: u32, y: u32, bit_to_embed: usize) {
    let mut pixel = img.get_pixel(x, y).clone();
    // only embed in red
    pixel.data[0] = embed_bit_into_byte(pixel.data[0], bit_to_embed);
    img.put_pixel(x, y, pixel);
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

