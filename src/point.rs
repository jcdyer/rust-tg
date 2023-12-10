//! Point type and associated code.
//!
//! - [x] Create type and field accessors
//! - [x] Add tg_sys conversions
//! - [x] Add PointFuncs
//! - [x] Standard traits
//! - [ ] Documentation
//! - [ ] Serde traits

use crate::{Geom, Rect};
use core::fmt;
use tg_sys::{tg_point, GeometryConstructors, GeometryConstructorsEx, PointFuncs};

#[cfg(feature = "serde")]
use serde::{Deserializer, de::Visitor, ser::SerializeTuple, Deserialize, Serialize};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

/// Constructors and accessor methods
impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    pub fn to_raw(self) -> tg_point {
        tg_point {
            x: self.x,
            y: self.y,
        }
    }

    pub fn from_raw(raw: tg_point) -> Point {
        Point { x: raw.x, y: raw.y }
    }

    pub fn with_x(mut self, x: f64) -> Point {
        self.x = x;
        self
    }

    pub fn with_y(mut self, y: f64) -> Point {
        self.y = y;
        self
    }
}

/// Operations defined in PointFuncs in tg.h and conversions
impl Point {
    pub fn rect(self) -> Rect {
        unsafe { PointFuncs::tg_point_rect(self.to_raw()).into() }
    }

    pub fn intersects_rect(self, rect: Rect) -> bool {
        unsafe { PointFuncs::tg_point_intersects_rect(self.to_raw(), rect.into()) }
    }

    pub fn geom(self) -> Geom {
        unsafe { GeometryConstructors::tg_geom_new_point(self.to_raw()) }.into()
    }

    pub fn geom_with_m(self, m: f64) -> Geom {
        unsafe { GeometryConstructorsEx::tg_geom_new_point_m(self.to_raw(), m) }.into()
    }

    pub fn geom_with_z(self, z: f64) -> Geom {
        unsafe { GeometryConstructorsEx::tg_geom_new_point_z(self.to_raw(), z) }.into()
    }

    pub fn geom_with_zm(self, z: f64, m: f64) -> Geom {
        unsafe { GeometryConstructorsEx::tg_geom_new_point_zm(self.to_raw(), z, m) }.into()
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

impl Default for Point {
    fn default() -> Self {
        Point::new(0., 0.)
    }
}
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl From<Point> for tg_point {
    fn from(value: Point) -> tg_point {
        value.to_raw()
    }
}

impl From<tg_point> for Point {
    fn from(value: tg_point) -> Point {
        Point::from_raw(value)
    }
}

#[cfg(feature = "serde")]
impl Serialize for Point {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut tuple = serializer.serialize_tuple(2)?;
        tuple.serialize_element(&self.x)?;
        tuple.serialize_element(&self.y)?;
        tuple.end()
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Point {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = Point;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("a pair of floating point numbers or an {x, y} object")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut x = None;
                let mut y = None;
                while let Some((k, v)) = map.next_entry()? {
                    if k == "x" {
                        x = Some(v);
                    } else if k == "y" {
                        y = Some(v)
                    } else {
                        return Err(serde::de::Error::unknown_field(k, &["x", "y"]));
                    }
                }
                let (x, y) = match (x, y) {
                    (Some(x), Some(y)) => (x, y),
                    (None, _) => return Err(serde::de::Error::missing_field("x")),
                    (_, None) => return Err(serde::de::Error::missing_field("y")),
                };

                Ok(Point::new(x, y))
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let pt = Point::new(
                    seq.next_element()?
                        .ok_or_else(|| serde::de::Error::invalid_length(0, &V))?,
                    seq.next_element()?
                        .ok_or_else(|| serde::de::Error::invalid_length(1, &V))?,
                );
                if seq.next_element::<serde::de::IgnoredAny>()?.is_some() {
                    Err(serde::de::Error::invalid_length(3, &V))
                } else {
                    Ok(pt)
                }
            }
        }

        de.deserialize_any(V)
    }
}
