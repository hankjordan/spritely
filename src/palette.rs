use std::collections::HashMap;

use crate::color::Color;

pub type Palette<P> = HashMap<Color<P>, Color<P>>;
