use std::sync::Arc;

pub struct Scene {
    // lights: Vec<Arc<&Light>>,
    // world_bound: Bounds3f,
    // aggregate: Arc<&Primitive>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            // lights: vec![],
            // world_bound: Bounds3f::default(),
            // aggregate: &Arc::new(Primitive::default()),
        }
    }

    /*pub fn new_from_scene(aggregate: &Arc<Primitive>, lights: Vec<Arc<&Light>>) -> Self {
        for light in lights {
            light.preprocess();
        }
        Self {
            // aggregate
            // lights,
            // world_bound: aggregate.world_bound(),
        }
    }*/

    /*pub fn intersect(&self, ray: &mut ray, isect: &SurfaceIteraction) -> bool {
        return self.aggregate.Intersect(ray, isect);
    }*/
}
