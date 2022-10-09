use std::collections::HashMap;

use image::io::Reader as ImageReader;
use spritely::{recolor::*, color::Color};
fn main() {
    const FILE: &str = "examples/log.png";

    // In-place
    {
        let mut sprite = ImageReader::open(FILE).unwrap().decode().unwrap().into_rgba8();
        let mut palette = HashMap::new();
        palette.insert(Color::from_rgba(0, 0, 0, 0), Color::from_rgba(1, 1, 1, 1));

        println!("Colors (in-place): {:?}", sprite.colors());
        sprite.recolor(&palette);
        println!("Colors (in-place, remapped): {:?}", sprite.colors());

        println!();
    }

    // Consume
    {
        let sprite = ImageReader::open(FILE).unwrap().decode().unwrap().into_rgba8();
        let mut palette = HashMap::new();
        palette.insert(Color::from_rgba(0, 0, 0, 0), Color::from_rgba(1, 1, 1, 1));

        println!("Colors (consume): {:?}", sprite.colors());
        let sprite2 = sprite.into_recolored(&palette);
        println!("Colors (consume, remapped): {:?}", sprite2.colors());

        println!();
    }

    // Clone
    {
        let sprite = ImageReader::open(FILE).unwrap().decode().unwrap().into_rgba8();
        let mut palette = HashMap::new();
        palette.insert(Color::from_rgba(0, 0, 0, 0), Color::from_rgba(1, 1, 1, 1));

        println!("Colors (clone): {:?}", sprite.colors());
        let sprite2 = sprite.clone_recolored(&palette);
        println!("Colors (clone, original): {:?}", sprite.colors());
        println!("Colors (clone, remapped): {:?}", sprite2.colors()); 

        println!();
    }
}