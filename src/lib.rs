extern crate image;
extern crate rand;

use image::{ImageBuffer,Rgba};

use rand::{Rng, SeedableRng, StdRng}; 
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
    let (width, height) = output.dimensions();
    //println!("dimensions {} {}", width, height);
    let mut msg_len = message.len();
    // rng
    let seed: &[_] = &[1, 2, 3, 4];
    let mut rng: StdRng = SeedableRng::from_seed(seed);

    // embed msg len first
    for _i in 0..32 as u32 {
        let x = rng.gen_range::<u32>(0, width);
        let y = rng.gen_range::<u32>(0, height);
        let c = rng.gen_range::<usize>(0, 3);
        //println!("will embed into x:{}, y:{}, c:{}", x, y, c); 
        let bit_to_embed = msg_len % 2;
        msg_len = msg_len / 2;
        embed_bit_into_pixel(&mut output, x, y, c, bit_to_embed);

    }
    // iterate chars
    for ch in message.bytes() {
        //println!("{}", ch);
        let mut byte: u8 = ch as u8;
        // iterate bits
        for _i in 0..8 as u32 {
            let x = rng.gen_range::<u32>(0, width);
            let y = rng.gen_range::<u32>(0, height);
            let c = rng.gen_range::<usize>(0, 3);
            let bit_to_embed = byte % 2;
            byte = byte / 2;
            //println!("  {}, x:{}, y:{}, c:{}", bit_to_embed, x, y, c);
            embed_bit_into_pixel(&mut output, x, y, c, bit_to_embed as usize);
        }
    }
    println!("+++{}+++", message.len());
    output
}

pub fn lsb_extract(steganogram: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> String {
    let (width, height) = steganogram.dimensions();
    // extract message length
    let mut msg_len: u32 = 0;
    // rng
    let seed: &[_] = &[1, 2, 3, 4];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    for i in 0..32 {
        let x = rng.gen_range::<u32>(0, width);
        let y = rng.gen_range::<u32>(0, height);
        let c = rng.gen_range::<usize>(0, 3);
        let bit = extract_bit_from_pixel(&steganogram, x, y, c);
        msg_len += bit as u32 * 2_u32.pow(i);
    }
    println!("---{}---", msg_len);
    // extract char by char
    let mut message: String = String::new();
    for _char_idx in 0..msg_len {
        let mut byte: u8 = 0;
        for i in 0..8 {
            let x = rng.gen_range::<u32>(0, width);
            let y = rng.gen_range::<u32>(0, height);
            let c = rng.gen_range::<usize>(0, 3);
            let bit = extract_bit_from_pixel(&steganogram, x, y, c);
            //println!("  {}, x:{}, y:{}, c:{}", bit, x, y, c);
            byte += bit * 2_u8.pow(i);
        }
        message.push(byte as char);
    }
    message
}

fn embed_bit_into_pixel(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, x: u32, y: u32, c: usize, bit_to_embed: usize) {
    let mut pixel = img.get_pixel(x, y).clone();
    // only embed in red
    pixel.data[c] = embed_bit_into_byte(pixel.data[c], bit_to_embed);
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

fn extract_bit_from_pixel(img: &ImageBuffer<Rgba<u8>, Vec<u8>>, x: u32, y: u32, c: usize) -> u8 {
    let pixel = img.get_pixel(x, y);
    pixel.data[c] % 2
}
