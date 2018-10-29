extern crate copper;
extern crate image;

use std::io;

fn main() {
    let mut message = String::new();
    println!("Enter message to embed:");
    io::stdin().read_line(&mut message).expect("Error reading input");

    let cover = image::open("avatar.png".to_string()).unwrap().to_rgba();
    let output = copper::lsb_embed(&cover, &message, &"Geheim".to_string());
    output.save("test.png").unwrap();
    
    let msg = copper::lsb_extract(&output, &"Geheim".to_string());
    println!("{}", msg);
}
