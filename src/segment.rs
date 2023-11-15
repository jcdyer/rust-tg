//! Segment type and associated code
//!
//! - [x] Create type and field accessors
//! - [x] Add tg_sys conversions
//! - [x] Add SegmentFuncs
//! - [x] Standard traits
//! - [ ] Serde traits
//! - [ ] Should reversed segments be equal?
//! - [ ] Documentation
//!
use core::fmt;

use tg_sys::{tg_segment, SegmentFuncs};

use crate::{Point, Rect};

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Segment {
    inner: tg_segment,
}

/// Constructors and accessor methods
impl Segment {
    pub fn new(a: Point, b: Point) -> Segment {
        Segment {
            inner: tg_segment {
                a: a.into(),
                b: b.into(),
            },
        }
    }

    pub fn to_raw(self) -> tg_segment {
        self.inner
    }

    pub fn from_raw(value: tg_segment) -> Segment {
        Segment { inner: value }
    }

    pub fn a(self) -> Point {
        self.inner.a.into()
    }

    pub fn b(self) -> Point {
        self.inner.b.into()
    }

    pub fn set_a(&mut self, a: Point) {
        self.inner.a = a.to_raw();
    }

    pub fn set_b(&mut self, b: Point) {
        self.inner.b = b.to_raw();
    }

    pub fn with_a(mut self, a: Point) -> Segment {
        self.set_a(a);
        self
    }

    pub fn with_b(mut self, b: Point) -> Segment {
        self.set_b(b);
        self
    }
}

/// Operations defined in SegmentFuncs in tg.h
impl Segment {
    pub fn rect(self) -> Rect {
        unsafe { SegmentFuncs::tg_segment_rect(self.to_raw()) }.into()
    }

    pub fn intersects_segment(self, segment: Segment) -> bool {
        unsafe { SegmentFuncs::tg_segment_intersects_segment(self.to_raw(), segment.to_raw()) }
    }
}

impl Default for Segment {
    fn default() -> Self {
        Self::new(Point::default(), Point::default())
    }
}

impl fmt::Debug for Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Segment")
            .field("a", &self.a())
            .field("b", &self.b())
            .finish()
    }
}

impl PartialEq for Segment {
    fn eq(&self, other: &Self) -> bool {
        self.a() == other.a() && self.b() == other.b()
    }
}

impl From<Segment> for tg_segment {
    fn from(value: Segment) -> tg_segment {
        value.to_raw()
    }
}

impl From<tg_segment> for Segment {
    fn from(value: tg_segment) -> Segment {
        Segment::from_raw(value)
    }
}
