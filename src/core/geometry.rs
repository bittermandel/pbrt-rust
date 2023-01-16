use std::ops::Index;

pub struct RayDifferential {}

pub struct Vector2i {
    pub x: i32,
    pub y: i32,
}

impl Vector2i {
    pub fn length_squared(&self) -> i32 {
        return self.x * self.x + self.y * self.y;
    }
}

pub struct Vector2f {
    pub x: f32,
    pub y: f32,
}

impl Vector2f {
    pub fn has_nans(&self) -> bool {
        return self.x.is_nan() || self.y.is_nan();
    }

    pub fn length_squared(&self) -> f32 {
        return self.x * self.x + self.y * self.y;
    }

    pub fn length(&self) -> f32 {
        return self.length_squared().sqrt();
    }
}

impl Index<i32> for Vector2i {
    type Output = i32;

    fn index(&self, index: i32) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl Index<i32> for Vector2f {
    type Output = f32;

    fn index(&self, index: i32) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Index out of bounds"),
        }
    }
}

pub struct Vector3i {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Vector3i {
    pub fn length_squared(&self) -> i32 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }
}

pub struct Vector3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3f {
    pub fn length_squared(&self) -> f32 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }
}

pub struct Point2i {
    pub x: i32,
    pub y: i32,
}

pub struct Point2f {
    pub x: f32,
    pub y: f32,
}

pub struct Bounds2i {
    pub p_min: Point2i,
    pub p_max: Point2i,
}

impl Bounds2i {
    pub fn diagonal(&self) -> Vector2i {
        return Vector2i {
            x: self.p_max.x - self.p_min.x,
            y: self.p_max.y - self.p_min.y,
        };
    }
}

pub struct Bounds2f {
    pub p_min: Point2f,
    pub p_max: Point2f,
}

impl Bounds2f {
    pub fn diagonal(&self) -> Vector2f {
        return Vector2f {
            x: self.p_max.x - self.p_min.x,
            y: self.p_max.y - self.p_min.y,
        };
    }
}
