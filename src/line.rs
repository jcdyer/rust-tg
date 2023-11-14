//! Line type and associated code
//!
//! - [x] Create type and constructors
//! - [x] Make Send + Sync
//! - [ ] impl Drop
//! - [ ] Create accessors
//! - [ ] Add tg_sys conversions
//! - [ ] Add Geom conversions
//! - [ ] Add LineFuncs
//! - [ ] Standard traits
//! - [ ] Documentation

use tg_sys::{tg_line, tg_point, LineFuncs};

use crate::{Point, IndexType};

pub struct Line {
    inner: *mut tg_line,
}

/// Constructors
impl Line {
    pub fn new(points: &[Point]) -> Line {
        let ptr = points.as_ptr() as *const tg_point;
        let len = points.len().try_into().unwrap();
        unsafe {
            Line {
                inner: LineFuncs::tg_line_new(ptr, len),
            }
        }
    }

    pub fn new_indexed(points: &[Point], index: IndexType) -> Line {
        let ptr = points.as_ptr() as *const tg_point;
        let len = points.len().try_into().unwrap();
        unsafe {
            Line {
                inner: LineFuncs::tg_line_new_ix(ptr, len, index.into()),
            }
        }
    }
}

/// Operations from LineFuncs
impl Line {
    pub fn length(&self) -> f64 {
        unsafe { LineFuncs::tg_line_length(self.inner) }
    }

    pub fn num_points(&self) -> usize {
        unsafe { LineFuncs::tg_line_num_points(self.inner) }
            .try_into()
            .unwrap()
    }

    pub fn num_segments(&self) -> usize {
        unsafe { LineFuncs::tg_line_num_segments(self.inner) }
            .try_into()
            .unwrap()
    }
}

#[cfg(feature = "atomics")]
unsafe impl Send for Line {}
#[cfg(feature = "atomics")]
unsafe impl Sync for Line {}

impl Drop for Line {
    fn drop(&mut self) {
        unsafe { LineFuncs::tg_line_free(self.inner) }
    }
}
