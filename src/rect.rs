//! Rectangle type and associated code
//!
//! - [x] Create type and field accessors
//! - [x] Add tg_sys conversions
//! - [x] Add RectFuncs
//! - [ ] Standard traits
//! - [ ] Documentation


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

    pub fn min(self) -> Point {
        self.inner.min.into()
    }

    pub fn max(self) -> Point {
        self.inner.max.into()
    }

    pub fn set_min(&mut self, min: Point) {
        self.inner.min = min.into();
    }
    pub fn set_max(&mut self, max: Point) {
        self.inner.max = max.into();
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
    pub fn center(self) -> Point{
        unsafe {
            RectFuncs::tg_rect_center(self.inner)
        }.into()
    }

    pub fn expand(self, other: Rect) -> Rect {
        unsafe {
            RectFuncs::tg_rect_expand(self.inner, other.inner)
        }.into()
    }

    pub fn expand_point(self, other: Point) -> Rect {
        unsafe {
            RectFuncs::tg_rect_expand_point(self.inner, other.into())
        }.into()

    }

    pub fn intersects_point(self, other: Point) -> bool {
        unsafe {
            RectFuncs::tg_rect_intersects_point(self.inner, other.into())
        }
    }

    pub fn intersects_rect(self, other: Rect) -> bool{
        unsafe {
            RectFuncs::tg_rect_intersects_rect(self.inner, other.inner)
        }
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
