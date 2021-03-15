use std::ops;

pub type Point3D = Vec3;
pub type Color = Vec3;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Vec3 {
    pub x: f32, 
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 {
            x: x, 
            y: y, 
            z: z
        }
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn dot(&self, rhs: &Vec3) -> f32 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z) 
    }

    pub fn unit_vector(&self) -> Vec3 {
        self / self.length()
    }
}

impl ops::Neg for Vec3 {
    type Output = Self; 
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Add::<f32> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f32) -> Vec3 {
        Vec3 {
            x: self.x + rhs, 
            y: self.y + rhs, 
            z: self.z + rhs,
        }
    }
}

impl ops::Add::<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn add(self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::AddAssign::<f32> for Vec3 {
    fn add_assign(&mut self, rhs: f32) {        
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;

    }
}

impl ops::AddAssign::<&Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: &Vec3) {        
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Sub::<f32> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: f32) -> Vec3 {
        Vec3 {
            x: self.x - rhs, 
            y: self.y - rhs, 
            z: self.z - rhs,
        }
    }
}

impl ops::Sub::<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x, 
            y: self.y - rhs.y, 
            z: self.z - rhs.z
        }
    }
}

impl ops::SubAssign::<f32> for Vec3 {
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs;
        self.y -= rhs; 
        self.z -= rhs;
    }
}

impl ops::SubAssign::<&Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: &Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y; 
        self.z -= rhs.z;
    }
}

impl ops::Mul::<f32> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Vec3 {
        Vec3 {
            x: self.x * rhs, 
            y: self.y * rhs, 
            z: self.z * rhs,
        }
    }
}

impl ops::Mul::<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x * rhs.x, 
            y: self.y * rhs.y, 
            z: self.z * rhs.z
        }
    }
}

impl ops::MulAssign::<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::MulAssign::<&Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: &Vec3) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl ops::Div::<f32> for &Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f32) -> Vec3 {
        Vec3 {
            x: self.x / rhs, 
            y: self.y / rhs, 
            z: self.z / rhs,
        }
    }
}

impl ops::Div::<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn div(self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl ops::DivAssign::<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl ops::DivAssign::<&Vec3> for Vec3 {
    fn div_assign(&mut self, rhs: &Vec3) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}
