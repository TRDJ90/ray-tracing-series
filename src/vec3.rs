use std::ops;

pub type Point3D = Vec3;
pub type Color = Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    x: f32, 
    y: f32,
    z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 {
            x: x, 
            y: y, 
            z: z
        }
    }

    pub fn get_x_coord(&self) -> f32 {
        self.x
    }
    
    pub fn get_y_coord(&self) -> f32 {
        self.y
    } 

    pub fn get_z_coord(&self) -> f32 {
        self.z
    } 

    fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    fn length_squared(&self) -> f32 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }
}

#[inline]
pub fn cross(lhs: &Vec3, rhs: &Vec3) -> Vec3 {
    Vec3 {
        x: lhs.y * rhs.z - lhs.z * rhs.y,
        y: lhs.z * rhs.x - lhs.x * rhs.z,
        z: lhs.x * rhs.y - lhs.y * rhs.x,
    }
}

#[inline]
pub fn dot(lhs: &Vec3, rhs: &Vec3) -> f32 {
    (lhs.x * rhs.x) + (lhs.y * rhs.y) + (lhs.z * rhs.z) 
}

#[inline]
pub fn unit_vector(vec3: Vec3) -> Vec3 {
    vec3 / vec3.length()
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        *self = Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x, 
            y: self.y + rhs.y, 
            z: self.z + rhs.z
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x, 
            y: self.y - rhs.y, 
            z: self.z - rhs.z
        }
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x, 
            y: self.y * rhs.y, 
            z: self.z * rhs.z
        }
    }
}

impl ops::Mul::<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs, 
            y: self.y * rhs, 
            z: self.z + rhs,
        }
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: rhs.x * self, 
            y: rhs.y * self, 
            z: rhs.z + self,
        }
    }
}

impl ops::Div::<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self {
            x: self.x / rhs, 
            y: self.y / rhs, 
            z: self.z / rhs,
        }
    }
}