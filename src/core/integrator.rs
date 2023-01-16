use super::{
    camera::Camera,
    geometry::{Point2i, RayDifferential},
    sampler::Sampler,
    scene::Scene,
    spectrum::Spectrum,
};

trait Integrator {
    fn render(&self, scene: &Scene);
}

pub struct SamplerIntegrator {
    pub sampler: Sampler,
    pub camera: Camera,
}

impl SamplerIntegrator {
    pub fn new(camera: Camera, sampler: Sampler) -> Self {
        Self { camera, sampler }
    }

    pub fn preprocess(&self, scene: &Scene) {}

    pub fn Li(&self, ray: &RayDifferential, scene: &Scene, depth: i64) -> Spectrum {
        return Spectrum {};
    }
}

impl Integrator for SamplerIntegrator {
    fn render(&self, scene: &Scene) {
        self.preprocess(scene);

        // Render image tiles in parallel
        let sample_bounds = self.camera.film.get_sample_bounds();
        let sample_extent = sample_bounds.diagonal();
        let tile_size = 16;
        let n_tiles = Point2i {
            x: (sample_extent.x + tile_size - 1) / tile_size,
            y: (sample_extent.y + tile_size - 1) / tile_size,
        };

        let num_x_tiles = (n_tiles.x * tile_size) as u32;
        let num_y_tiles = (n_tiles.y * tile_size) as u32;

        let tiles: Vec<(u32, u32)> = (0..num_x_tiles * num_y_tiles)
            .map(|i| (i % num_x_tiles, i / num_x_tiles))
            .collect();

        println!("Rendering {:?} tiles", tiles);
    }
}
