use std::{fmt, ops::Mul};

use nalgebra::{Matrix2, Vector2};
use serde::{
    Deserialize,
    Deserializer,
    Serialize,
    Serializer,
    de,
    de::{SeqAccess, Visitor},
    ser::SerializeTuple,
};
use usvg::tiny_skia_path;

use crate::types::coord::Coord;

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct Transform {
    #[serde(
        deserialize_with = "deserialize_matrix2",
        serialize_with = "serialize_matrix2"
    )]
    pub(crate) rotate: Matrix2<f32>,
    pub(crate) offset: Coord,
}

fn deserialize_matrix2<'de, D>(data: D) -> Result<Matrix2<f32>, D::Error>
where
    D: Deserializer<'de>,
{
    struct PointVisitor;

    impl<'de> Visitor<'de> for PointVisitor {
        type Value = Matrix2<f32>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a tuple of 4 floats")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Matrix2<f32>, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let m11: f32 = seq
                .next_element()?
                .ok_or_else(|| de::Error::invalid_length(0, &self))?;
            let m12: f32 = seq
                .next_element()?
                .ok_or_else(|| de::Error::invalid_length(0, &self))?;
            let m21: f32 = seq
                .next_element()?
                .ok_or_else(|| de::Error::invalid_length(0, &self))?;
            let m22: f32 = seq
                .next_element()?
                .ok_or_else(|| de::Error::invalid_length(0, &self))?;

            Ok(Matrix2::new(m11, m12, m21, m22))
        }
    }
    data.deserialize_tuple(4, PointVisitor)
}

fn serialize_matrix2<S>(matrix: &Matrix2<f32>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut tup = serializer.serialize_tuple(4)?;
    tup.serialize_element(&matrix.m11)?;
    tup.serialize_element(&matrix.m12)?;
    tup.serialize_element(&matrix.m21)?;
    tup.serialize_element(&matrix.m22)?;
    tup.end()
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            rotate: Matrix2::new(1.0, 0.0, 0.0, 1.0),
            offset: Coord(0.0, 0.0),
        }
    }
}

impl From<&tiny_skia_path::Transform> for Transform {
    fn from(v: &usvg::Transform) -> Self {
        Transform {
            rotate: Matrix2::new(v.sx, v.kx, v.ky, v.sy),
            offset: Coord(v.tx, v.ty),
        }
    }
}

impl Mul<&Coord> for Matrix2<f32> {
    type Output = Coord;

    fn mul(self, rhs: &Coord) -> Coord {
        let vec = Vector2::new(rhs.0, rhs.1);
        let res = self * vec;
        Coord(res.x, res.y)
    }
}

impl Transform {
    pub(crate) fn apply(&self, coord: &Coord) -> Coord {
        self.rotate * coord + self.offset
    }

    pub fn rotate(&mut self, degrees: f32) {
        let radians = degrees * std::f32::consts::PI / 180.0;
        let cos = radians.cos();
        let sin = radians.sin();
        let rot = Matrix2::new(cos, sin, -sin, cos);
        self.rotate = rot * self.rotate;
    }

    pub fn scale(&mut self, factor: f32) {
        let scale = Matrix2::new(factor, 0.0, 0.0, factor);
        self.rotate = scale * self.rotate;
        self.offset.0 *= factor;
        self.offset.1 *= factor;
    }

    pub fn translate(&mut self, dx: f32, dy: f32) {
        self.offset.0 += dx;
        self.offset.1 += dy;
    }

    pub fn reset(&mut self) {
        self.offset = Coord(0.0, 0.0);
        self.rotate = Matrix2::new(1.0, 0.0, 0.0, 1.0);
    }
}
