extern crate image;
extern crate rand;
extern crate sha3;

use image::{ImageBuffer, Rgba};
use sha3::{Digest, Sha3_256};
use std::collections::HashSet;

use rand::{Rng, SeedableRng, StdRng};
#[cfg(test)]
mod tests {

    use crate::{lsb_embed, lsb_extract};

    #[test]
    fn it_works() {
        let test_message = "My hovercraft is full of eels!".to_string();
        let passphrase = "Foobar123".to_string();
        let cover = image::open("test.png".to_string()).unwrap().to_rgba();
        let output = lsb_embed(&cover, &test_message, &passphrase);
        assert_eq!(output.is_ok(), true);
        let output = output.unwrap();
        output.save("stego.png").unwrap();
        let extracted_message = lsb_extract(&output, &passphrase);
        assert_eq!(extracted_message.is_ok(), true);
        assert_eq!(test_message, extracted_message.unwrap());
    }
}

pub fn lsb_embed(
    cover: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    message: &String,
    passphrase: &String,
) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, String> {
    let mut output = cover.clone();
    let (width, height) = output.dimensions();
    //println!("dimensions {} {}", width, height);
    let mut msg_len = message.len();
    let embeddable_len = width * height * 3 / 10; // only embed in a max of 10 percent of possible pixels
    if msg_len * 8 + 32 > embeddable_len as usize {
        return Err("Message too long for image".to_string());
    }
    let mut already_embedded: HashSet<(u32, u32, usize)> = HashSet::new();
    let mut rng = get_rng(&passphrase);

    // embed msg len first
    for _i in 0..32 as u32 {
        let (x, y, c) = get_free_pixel(&mut rng, &mut already_embedded, width, height);
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
            let (x, y, c) = get_free_pixel(&mut rng, &mut already_embedded, width, height);
            let bit_to_embed = byte % 2;
            byte = byte / 2;
            //println!("  {}, x:{}, y:{}, c:{}", bit_to_embed, x, y, c);
            embed_bit_into_pixel(&mut output, x, y, c, bit_to_embed as usize);
        }
    }
    println!("+++{}+++", message.len());
    Ok(output)
}

pub fn lsb_extract(
    steganogram: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    passphrase: &String,
) -> Result<String, String> {
    let (width, height) = steganogram.dimensions();
    // extract message length
    let mut msg_len: u32 = 0;
    let mut already_embedded: HashSet<(u32, u32, usize)> = HashSet::new();
    let mut rng = get_rng(&passphrase);
    for i in 0..32 {
        let (x, y, c) = get_free_pixel(&mut rng, &mut already_embedded, width, height);
        let bit = extract_bit_from_pixel(&steganogram, x, y, c);
        msg_len += bit as u32 * 2_u32.pow(i);
    }
    let embeddable_len = width * height * 3 / 10; // only embed in a max of 10 percent of possible pixels
    if msg_len * 8 + 32 > embeddable_len {
        return Err(
            "Something went wrong on extracting. Wrong passphrase? No message embedded?"
                .to_string(),
        );
    }
    println!("---{}---", msg_len);
    // extract char by char
    let mut message: String = String::new();
    for _char_idx in 0..msg_len {
        let mut byte: u8 = 0;
        for i in 0..8 {
            let (x, y, c) = get_free_pixel(&mut rng, &mut already_embedded, width, height);
            let bit = extract_bit_from_pixel(&steganogram, x, y, c);
            //println!("  {}, x:{}, y:{}, c:{}", bit, x, y, c);
            byte += bit * 2_u8.pow(i);
        }
        message.push(byte as char);
    }
    Ok(message)
}

fn get_rng(passphrase: &String) -> StdRng {
    // rng
    let mut hasher = Sha3_256::new();
    hasher.input(passphrase);
    let hash = hasher.result();
    let seed: &[_] = &[
        hash[0] as usize,
        hash[1] as usize,
        hash[2] as usize,
        hash[3] as usize,
        hash[4] as usize,
        hash[5] as usize,
        hash[6] as usize,
        hash[7] as usize,
        hash[8] as usize,
        hash[9] as usize,
        hash[10] as usize,
        hash[11] as usize,
        hash[12] as usize,
        hash[13] as usize,
        hash[14] as usize,
        hash[15] as usize,
        hash[16] as usize,
        hash[17] as usize,
        hash[18] as usize,
        hash[19] as usize,
        hash[20] as usize,
        hash[21] as usize,
        hash[22] as usize,
        hash[23] as usize,
        hash[24] as usize,
        hash[25] as usize,
        hash[26] as usize,
        hash[27] as usize,
        hash[28] as usize,
        hash[29] as usize,
        hash[30] as usize,
        hash[31] as usize,
    ];
    let rng: StdRng = SeedableRng::from_seed(seed);
    rng
}

fn get_free_pixel(
    rng: &mut StdRng,
    already_embedded: &mut HashSet<(u32, u32, usize)>,
    width: u32,
    height: u32,
) -> (u32, u32, usize) {
    let mut x = rng.gen_range::<u32>(0, width);
    let mut y = rng.gen_range::<u32>(0, height);
    let mut c = rng.gen_range::<usize>(0, 3);
    while already_embedded.contains(&(x, y, c)) {
        x = rng.gen_range::<u32>(0, width);
        y = rng.gen_range::<u32>(0, height);
        c = rng.gen_range::<usize>(0, 3);
    }
    already_embedded.insert((x, y, c));
    (x, y, c)
}

fn embed_bit_into_pixel(
    img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    x: u32,
    y: u32,
    c: usize,
    bit_to_embed: usize,
) {
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
