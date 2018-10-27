extern crate copper;
extern crate image;

fn main() {
    let cover = image::open("avatar.png".to_string()).unwrap().to_rgba();
    let output = copper::lsb_embed(&cover, &"My hovercraft is full of eels!".to_string());
    output.save("test.png").unwrap();
    
    let msg = copper::lsb_extract(&output);
    println!("{}", msg);
}
