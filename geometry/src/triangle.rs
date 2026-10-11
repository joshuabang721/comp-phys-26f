use crate::vector3::{Vector3, cross};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct Triangle {
    pub vertex: Vector3,
    pub edge1: Vector3,
    pub edge2: Vector3,
    pub norm: Vector3,
}

impl Triangle {
    pub fn vertices(&self) -> [Vector3; 3] {
        [
            self.vertex,
            self.vertex + self.edge1,
            self.vertex + self.edge2,
        ]
    }
}

pub fn newnewnewnewnew(vertex: Vector3, edge1: Vector3, edge2: Vector3) -> Option<Triangle> {
    Some(Triangle {
        vertex,
        edge1,
        edge2,
        norm: cross(edge1, edge2).normalized()?,
    })
}
