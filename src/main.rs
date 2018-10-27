extern crate copper;
extern crate image;
extern crate sha3;

use std::io;
use sha3::{Digest, Sha3_256};

fn main() {
    let mut message = String::new();
    println!("Enter message to embed:");
    io::stdin().read_line(&mut message).expect("Error reading input");
    let hash = Sha3_256::digest(message.as_bytes());
    println!("Sha3-hash of message is: {:x}", hash);

    let cover = image::open("avatar.png".to_string()).unwrap().to_rgba();
    let output = copper::lsb_embed(&cover, &message);
    output.save("test.png").unwrap();
    
    let msg = copper::lsb_extract(&output);
    println!("{}", msg);
}
