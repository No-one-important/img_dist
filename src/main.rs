extern crate image;
extern crate img_hash;
 
use img_hash::HasherConfig;
use std::env::args;

fn main() {
    let image1 = image::open(args().nth(1).unwrap()).unwrap();
    let image2 = image::open(args().nth(2).unwrap()).unwrap();

    let hasher = HasherConfig::new().to_hasher();

    let hash1 = hasher.hash_image(&image1);
    let hash2 = hasher.hash_image(&image2);

    println!("Image1 hash: {}", hash1.to_base64());
    println!("Image2 hash: {}", hash2.to_base64());

    println!("Hamming Distance: {}", hash1.dist(&hash2));
}