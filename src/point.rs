//! Point type and associated code.
//!
//! - [x] Create type and field accessors
//! - [x] Add tg_sys conversions
//! - [x] Add PointFuncs
//! - [x] Standard traits
//! - [ ] Documentation
//! - [ ] Serde traits

use core::fmt;

use tg_sys::{tg_point, GeometryConstructors, GeometryConstructorsEx, PointFuncs};

use crate::{Geom, Rect};

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Point {
    inner: tg_point,
}

/// Constructors and accessor methods
impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point {
            inner: tg_point { x, y },
        }
    }

    pub fn to_raw(self) -> tg_point {
        self.inner
    }

    pub fn from_raw(raw: tg_point) -> Point {
        Point { inner: raw }
    }

    pub fn x(self) -> f64 {
        self.to_raw().x
    }

    pub fn y(self) -> f64 {
        self.to_raw().y
    }

    pub fn set_x(&mut self, x: f64) {
        self.inner.x = x;
    }

    pub fn set_y(&mut self, y: f64) {
        self.inner.x = y;
    }

    pub fn with_x(mut self, x: f64) -> Point {
        self.set_x(x);
        self
    }

    pub fn with_y(mut self, y: f64) -> Point {
        self.set_y(y);
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
            .field("x", &self.x())
            .field("y", &self.y())
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
        self.x() == other.x() && self.y() == other.y()
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
