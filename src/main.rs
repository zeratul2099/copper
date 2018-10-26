extern crate copper;


fn main() {
    copper::lsb_embed("avatar.png".to_string(), "My hovercraft is full of eels!".to_string());
    copper::lsb_extract("avatar.png".to_string());
}
