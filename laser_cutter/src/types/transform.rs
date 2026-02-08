use std::{f32::consts::PI, fmt, ops::Mul};

use anyhow::bail;
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
use svg::node::Value;

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

impl TryFrom<&Value> for Transform {
    type Error = anyhow::Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let value_str = value.to_string().replace(")", "");
        let parts = value_str.split_once("(");
        if let Some((form, values)) = parts {
            let v = values
                .split(",")
                .map(|s| s.trim().parse::<f32>())
                .collect::<Result<Vec<_>, _>>()?;
            match form {
                "matrix" => Ok(Transform {
                    rotate: Matrix2::new(v[0], v[1], v[2], v[3]),
                    offset: Coord(v[4], v[5]),
                }),
                "translate" => Ok(Transform {
                    rotate: Matrix2::new(1.0, 0.0, 0.0, 1.0),
                    offset: Coord(v[0], v[1]),
                }),
                "scale" => {
                    let scale_x = v[0];
                    let scale_y = match v.get(1) {
                        Some(v) => *v,
                        None => scale_x,
                    };
                    Ok(Transform {
                        rotate: Matrix2::new(scale_x, 0.0, 0.0, scale_y),
                        offset: Coord::default(),
                    })
                }
                "rotate" => {
                    // rotate around point not supported, yet.
                    let rad = v[0] * PI / 180.0;
                    Ok(Transform {
                        rotate: Matrix2::new(rad.cos(), -rad.sin(), rad.sin(), rad.cos()),
                        offset: Coord::default(),
                    })
                }
                "skewX" => bail!("skewX not supported yet"),
                "skewY" => bail!("skewY not supported yet"),
                _ => bail!("unsupported transform: {}", value),
            }
        } else {
            bail!("invalid transform value: {}", value);
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

#[cfg(test)]
mod tests {
    use std::f32::consts::FRAC_1_SQRT_2;

    use nalgebra::Matrix2;
    use svg::node::Value;

    use crate::types::{coord::Coord, transform::Transform};

    #[test]
    fn test_transform_from_value_matrix() {
        let value = Value::from("matrix(1, 0, 0, -1, 28, 81)");
        let result = Transform::try_from(&value).unwrap();
        let expected = Transform {
            rotate: Matrix2::new(1.0, 0.0, 0.0, -1.0),
            offset: Coord(28.0, 81.0),
        };
        assert_eq!(expected, result);
    }
    #[test]
    fn test_transform_from_value_translate() {
        let value = Value::from("translate(51.47239,-877.48168)");
        let result = Transform::try_from(&value).unwrap();
        let expected = Transform {
            rotate: Matrix2::new(1.0, 0.0, 0.0, 1.0),
            offset: Coord(51.47239, -877.48168),
        };
        assert_eq!(expected, result);
    }

    #[test]
    fn test_transform_from_value_scale() {
        let value = Value::from("scale(51.47239)");
        let result = Transform::try_from(&value).unwrap();
        let expected = Transform {
            rotate: Matrix2::new(51.47239, 0.0, 0.0, 51.47239),
            offset: Coord::default(),
        };
        assert_eq!(expected, result);
    }

    #[test]
    fn test_transform_from_value_scale2() {
        let value = Value::from("scale(51.47239,47.452)");
        let result = Transform::try_from(&value).unwrap();
        let expected = Transform {
            rotate: Matrix2::new(51.47239, 0.0, 0.0, 47.452),
            offset: Coord::default(),
        };
        assert_eq!(expected, result);
    }

    #[test]
    fn test_transform_from_value_rotate() {
        let value = Value::from("rotate(45.0)");
        let result = Transform::try_from(&value).unwrap();
        let expected = Transform {
            rotate: Matrix2::new(FRAC_1_SQRT_2, -FRAC_1_SQRT_2, FRAC_1_SQRT_2, FRAC_1_SQRT_2),
            offset: Coord::default(),
        };
        assert_eq!(expected, result);
    }
}
