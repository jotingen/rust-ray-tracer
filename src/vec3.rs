use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

#[derive(Clone, Copy)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}
impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }
    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
    }
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
}
impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, v: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }
}
impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, v: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z,
        }
    }
}
impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * v.x,
            y: self.y * v.y,
            z: self.z * v.z,
        }
    }
}
pub trait Dot {
    fn dot(self, v: Vec3) -> f64;
}
impl Dot for Vec3 {
    fn dot(self, v: Vec3) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }
}
pub trait Cross {
    fn cross(self, v: Vec3) -> Vec3;
}
impl Cross for Vec3 {
    fn cross(self, v: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x,
        }
    }
}
pub trait UnitVector {
    fn unit_vector(self) -> Vec3;
}
impl UnitVector for Vec3 {
    fn unit_vector(self) -> Vec3 {
        self / self.length()
    }
}
impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, t: f64) -> Vec3 {
        Vec3 {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }
}
impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, t: f64) -> Vec3 {
        self * (1.0 / t)
    }
}
pub type Point3 = Vec3;
pub type Color = Vec3;
