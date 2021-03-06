use std::{fmt::Display, ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign}};
use crate::vector2::Vector2;
use crate::vector4::Vector4;

#[derive(Copy, Clone, Debug)]
pub struct Vector3<T: Default + Display + Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign + Mul<Output = T> + MulAssign + Div<Output = T> + DivAssign> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Default + Display + Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign + Mul<Output = T> + MulAssign + Div<Output = T> + DivAssign> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self {
            x,
            y,
            z
        }
    }

    pub fn empty() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
            z: Default::default(),
        }
    }

    pub fn from_vector2(vec2: Vector2<T>) -> Vector3<T> {
        Vector3::new(vec2.x, vec2.y, Default::default())
    }

    pub fn from_vector4(vec4: Vector4<T>) -> Vector3<T> {
        Vector3::new(vec4.x, vec4.y, vec4.z)
    }
}

impl<T: Default + Display + Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign + Mul<Output = T> + MulAssign + Div<Output = T> + DivAssign> Add for Vector3<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: Default + Display + Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign + Mul<Output = T> + MulAssign + Div<Output = T> + DivAssign> AddAssign for Vector3<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl <T: Default + Display + Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign + Mul<Output = T> + MulAssign + Div<Output = T> + DivAssign> Sub for Vector3<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: Default + Display + Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign + Mul<Output = T> + MulAssign + Div<Output = T> + DivAssign> SubAssign for Vector3<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T: Default + Display + Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign + Mul<Output = T> + MulAssign + Div<Output = T> + DivAssign> Mul for Vector3<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<T: Default + Display + Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign + Mul<Output = T> + MulAssign + Div<Output = T> + DivAssign> MulAssign for Vector3<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl<T: Default + Display + Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign + Mul<Output = T> + MulAssign + Div<Output = T> + DivAssign> Div for Vector3<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl<T: Default + Display + Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign + Mul<Output = T> + MulAssign + Div<Output = T> + DivAssign> DivAssign for Vector3<T> {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl<T: Default + Display + Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign + Mul<Output = T> + MulAssign + Div<Output = T> + DivAssign> Display for Vector3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{{}, {}, {}}}", self.x, self.y, self.z)
    }
}
