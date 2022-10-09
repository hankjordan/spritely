use std::{
    collections::BTreeSet,
    hash::Hash,
    ops::{Deref, DerefMut},
};

use image::{ImageBuffer, Pixel};

use crate::color::Color;
use crate::palette::Palette;

pub trait ImageColors<P: Pixel> {
    fn colors(&self) -> Vec<Color<P>>;
}

pub trait ImageRecolorMut<P: Pixel> {
    fn recolor(&mut self, palette: &Palette<P>);
    fn into_recolored(self, palette: &Palette<P>) -> Self;
}

pub trait ImageRecolorClone<P: Pixel> {
    fn clone_recolored(&self, palette: &Palette<P>) -> Self;
}

impl<P, Container> ImageColors<P> for ImageBuffer<P, Container>
where
    P: Pixel,
    Container: Deref<Target = [P::Subpixel]>,
{
    fn colors(&self) -> Vec<Color<P>> {
        let mut set = BTreeSet::new();

        for pixel in self.pixels() {
            set.insert(Color::from(*pixel));
        }

        set.into_iter().collect()
    }
}

impl<P, Container> ImageRecolorMut<P> for ImageBuffer<P, Container>
where
    P: Pixel,
    P::Subpixel: Hash,
    Container: DerefMut<Target = [P::Subpixel]>,
{
    fn recolor(&mut self, palette: &Palette<P>) {
        for pixel in self.pixels_mut() {
            if let Some(new) = palette.get(&Color::from(*pixel)) {
                pixel.apply2(&new.value(), |_, other| other);
            }
        }
    }

    fn into_recolored(mut self, palette: &Palette<P>) -> Self {
        self.recolor(palette);
        self
    }
}

impl<P, C> ImageRecolorClone<P> for ImageBuffer<P, C>
where
    P: Pixel,
    P::Subpixel: Hash,
    C: Deref<Target = [P::Subpixel]> + DerefMut<Target = [P::Subpixel]> + Clone,
{
    fn clone_recolored(&self, palette: &Palette<P>) -> Self {
        let mut result = self.clone();
        result.recolor(palette);
        result
    }
}
