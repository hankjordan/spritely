use std::hash::{Hash, Hasher};

use image::{Pixel, Rgb, Rgba};

#[derive(Debug)]
pub struct Color<P: Pixel> {
    value: P,
}

impl<P: Pixel> PartialEq for Color<P> {
    fn eq(&self, other: &Self) -> bool {
        self.value.channels() == other.value.channels()
    }
}

impl<P: Pixel> Eq for Color<P> {}

impl<P: Pixel> PartialOrd for Color<P> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.channels().partial_cmp(other.value.channels())
    }
}

impl<P: Pixel> Ord for Color<P> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.channels().partial_cmp(other.value.channels()).unwrap()
    }
}

impl<P: Pixel> Hash for Color<P>
where
    P::Subpixel: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        for item in self.value.channels() {
            item.hash(state);
        }
    }
}

impl<P: Pixel> From<P> for Color<P> {
    fn from(pixel: P) -> Self {
        Self { value: pixel }
    }
}

impl Color<Rgb<u8>> {
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self::from(Rgb([r, g, b]))
    }
}

impl Color<Rgba<u8>> {
    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::from(Rgba([r, g, b, a]))
    }
}

impl<P: Pixel> Color<P> {
    pub fn value(&self) -> P {
        self.value
    }
}
