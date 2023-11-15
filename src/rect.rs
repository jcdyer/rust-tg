//! Rectangle type and associated code
//!
//! - [x] Create type and field accessors
//! - [x] Add tg_sys conversions
//! - [x] Add RectFuncs
//! - [x] Standard traits
//! - [ ] Serde traits
//! - [ ] Documentation

use core::fmt;

use tg_sys::{tg_rect, RectFuncs};

use crate::Point;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Rect {
    inner: tg_rect,
}

/// Constructors and accessors
impl Rect {
    pub fn new(min: Point, max: Point) -> Rect {
        Rect {
            inner: tg_rect {
                min: min.into(),
                max: max.into(),
            },
        }
    }

    pub fn into_raw(self) -> tg_rect {
        self.inner
    }

    pub fn from_raw(raw: tg_rect) -> Rect {
        Rect { inner: raw }
    }

    pub fn min(self) -> Point {
        self.into_raw().min.into()
    }

    pub fn max(self) -> Point {
        self.into_raw().max.into()
    }

    pub fn set_min(&mut self, min: Point) {
        self.inner.min = min.into_raw();
    }
    pub fn set_max(&mut self, max: Point) {
        self.inner.max = max.into_raw();
    }
    pub fn with_min(mut self, min: Point) -> Rect {
        self.set_min(min);
        self
    }
    pub fn with_max(mut self, max: Point) -> Rect {
        self.set_max(max);
        self
    }
}

/// Operations defined in RectFuncs in tg.h
impl Rect {
    pub fn center(self) -> Point {
        unsafe { RectFuncs::tg_rect_center(self.into_raw()) }.into()
    }

    pub fn expand(self, other: Rect) -> Rect {
        unsafe { RectFuncs::tg_rect_expand(self.into_raw(), other.into_raw()) }.into()
    }

    pub fn expand_point(self, other: Point) -> Rect {
        unsafe { RectFuncs::tg_rect_expand_point(self.into_raw(), other.into_raw()) }.into()
    }

    pub fn intersects_point(self, other: Point) -> bool {
        unsafe { RectFuncs::tg_rect_intersects_point(self.into_raw(), other.into_raw()) }
    }

    pub fn intersects_rect(self, other: Rect) -> bool {
        unsafe { RectFuncs::tg_rect_intersects_rect(self.into_raw(), other.into_raw()) }
    }
}

impl fmt::Debug for Rect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Rect")
            .field("min", &self.min())
            .field("max", &self.max())
            .finish()
    }
}

impl Default for Rect {
    fn default() -> Self {
        Self::new(Point::new(0., 0.), Point::new(0., 0.))
    }
}

impl PartialEq for Rect {
    fn eq(&self, other: &Self) -> bool {
        self.min() == other.min() && self.max() == other.max()
    }
}

impl From<Rect> for tg_rect {
    fn from(value: Rect) -> tg_rect {
        value.inner
    }
}

impl From<tg_rect> for Rect {
    fn from(value: tg_rect) -> Rect {
        Rect { inner: value }
    }
}
