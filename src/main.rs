extern crate copper;
extern crate image;

use std::io;

fn main() {
    let mut message = String::new();
    println!("Enter message to embed:");
    io::stdin().read_line(&mut message).expect("Error reading input");

    let cover = image::open("test.png".to_string()).unwrap().to_rgba();
    let output = copper::lsb_embed(&cover, &message, &"foobar123".to_string()).unwrap();
    output.save("stego.png").unwrap();
    
    let msg = copper::lsb_extract(&output, &"foobar123".to_string()).unwrap();
    println!("{}", msg);
}
