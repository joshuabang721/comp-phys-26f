use crate::vector3::{self, Vector3};
use serde::{Deserialize, Serialize};
use std::ops;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct Quaternion {
    pub w: f64,
    pub v: Vector3,
}

impl ops::Neg for Quaternion {
    type Output = Quaternion;
    /// Quaternion conjugate
    ///
    /// q = w + v
    ///
    /// -q := q' = w - v
    fn neg(self) -> Self::Output {
        Quaternion {
            w: self.w,
            v: -self.v,
        }
    }
}

impl ops::Mul for Quaternion {
    type Output = Quaternion;
    fn mul(self, rhs: Self) -> Self::Output {
        Quaternion {
            w: self.w * rhs.w - self.v * rhs.v,
            v: self.w * rhs.v + self.v * rhs.w - vector3::cross(self.v, rhs.v),
        }
    }
}

impl ops::Mul<Vector3> for Quaternion {
    type Output = Vector3;
    /// For a vector v and unit quaternion q, v rotated by q (v') is:
    ///
    /// v' = q' * v * q
    fn mul(self, rhs: Vector3) -> Self::Output {
        (self * Quaternion { w: 0.0, v: rhs }).v
    }
}

impl ops::Mul<Quaternion> for Vector3 {
    type Output = Vector3;
    /// For a vector v and unit quaternion q, the vector v' obtained by rotating v with w is: is:
    ///
    /// v' = q' * v * q
    fn mul(self, rhs: Quaternion) -> Self::Output {
        (Quaternion { w: 0.0, v: self } * rhs).v
    }
}

pub fn newnewnewnew(t: f64, n: Vector3) -> Option<Quaternion> {
    let t2 = t / 2.0;
    Some(Quaternion {
        w: t2.cos(),
        v: t2.sin() * n.normalized()?,
    })
}
