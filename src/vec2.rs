use std::ops::{Add, Sub, Neg, Mul};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T
}

impl<T: Add<Output = T>> Add for Vec2<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl<T: Sub<Output = T>> Sub for Vec2<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl<T: Neg<Output = T>> Neg for Vec2<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Vec2 {
            x: -self.x,
            y: -self.y
        }
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Vec2<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl<T: Mul<Output = T> + Add<Output = T>> Vec2<T> {
    pub fn dot(self, other: Self) -> T {
        self.x * other.x + self.y * other.y
    }
}

impl Vec2<f32> {
    pub fn magnitude(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}