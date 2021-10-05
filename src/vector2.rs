use std::{fmt::Display, ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign}};

pub struct Vector2<T: Default + Display + Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign + Mul<Output = T> + MulAssign + Div<Output = T> + DivAssign> {
    pub x: T,
    pub y: T,
}

impl<T: Default + Display + Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign + Mul<Output = T> + MulAssign + Div<Output = T> + DivAssign> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }

    pub fn empty() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
        }
    }
}

impl<T: Default + Display + Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign + Mul<Output = T> + MulAssign + Div<Output = T> + DivAssign> Add for Vector2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Default + Display + Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign + Mul<Output = T> + MulAssign + Div<Output = T> + DivAssign> AddAssign for Vector2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl <T: Default + Display + Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign + Mul<Output = T> + MulAssign + Div<Output = T> + DivAssign> Sub for Vector2<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Default + Display + Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign + Mul<Output = T> + MulAssign + Div<Output = T> + DivAssign> SubAssign for Vector2<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Default + Display + Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign + Mul<Output = T> + MulAssign + Div<Output = T> + DivAssign> Mul for Vector2<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y
        }
    }
}

impl<T: Default + Display + Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign + Mul<Output = T> + MulAssign + Div<Output = T> + DivAssign> MulAssign for Vector2<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl<T: Default + Display + Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign + Mul<Output = T> + MulAssign + Div<Output = T> + DivAssign> Div for Vector2<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl<T: Default + Display + Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign + Mul<Output = T> + MulAssign + Div<Output = T> + DivAssign> DivAssign for Vector2<T> {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl<T: Default + Display + Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign + Mul<Output = T> + MulAssign + Div<Output = T> + DivAssign> Display for Vector2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{{}, {}}}", self.x, self.y)
    }
}