//! Point type and associated code.
//!
//! - [x] Create type and field accessors
//! - [x] Add tg_sys conversions
//! - [x] Add PointFuncs
//! - [ ] Standard traits
//! - [ ] Documentation

use tg_sys::{tg_point, PointFuncs};

use crate::Rect;

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

    pub fn x(self) -> f64 {
        self.inner.x
    }

    pub fn y(self) -> f64 {
        self.inner.y
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

/// Operations defined in PointFuncs in tg.h
impl Point {
    pub fn rect(self) -> Rect {
        unsafe { PointFuncs::tg_point_rect(self.inner).into() }
    }

    pub fn intersects_rect(self, rect: Rect) -> bool {
        unsafe { PointFuncs::tg_point_intersects_rect(self.inner, rect.into()) }
    }
}

impl From<Point> for tg_point {
    fn from(value: Point) -> tg_point {
        value.inner
    }
}

impl From<tg_point> for Point {
    fn from(value: tg_point) -> Point {
        Point { inner: value }
    }
}
