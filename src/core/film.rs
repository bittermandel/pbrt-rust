use super::geometry::{Bounds2i, Point2i};

struct Pixel {
    xyz: [f32; 3],
}

pub struct Film {
    diagonal: f32,
    full_resolution: f32,
    pixels: Vec<Pixel>,
}

impl Film {
    pub fn get_film_tile(sample_bounds: Bounds2i) -> FilmTile {
        return FilmTile {
            pixel_bounds: sample_bounds,
        };
    }

    pub fn get_sample_bounds(&self) -> Bounds2i {
        return Bounds2i {
            p_min: Point2i { x: 0, y: 0 },
            p_max: Point2i { x: 0, y: 0 },
        };
    }
}

pub struct FilmTile {
    pixel_bounds: Bounds2i,
}
