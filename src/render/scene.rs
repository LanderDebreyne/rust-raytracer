pub struct Scene {
    pub width: usize,
    pub height: usize,
    pub samples: u32,
    pub max_depth: u32,
}

impl Scene {
    pub fn new(width: usize, height: usize, samples: u32, max_depth: u32) -> Scene {
        Scene {
            width,
            height,
            samples,
            max_depth,
        }
    }
}