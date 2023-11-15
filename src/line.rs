//! Line type and associated code
//!
//! - [x] Create type and constructors
//! - [x] Make Send + Sync
//! - [x] impl Drop
//! - [x] Create accessors
//! - [ ] Add tg_sys conversions
//! - [ ] Add Geom conversions
//! - [x] Add LineFuncs
//! - [ ] Add LineFuncs "iterators"
//! - [ ] Standard traits
//! - [ ] Documentation

use std::marker::PhantomData;

use tg_sys::{tg_line, tg_point, LineFuncs};

use crate::{IndexType, Point, Rect, Segment};

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
    pub fn memsize(&self) -> usize {
        unsafe { LineFuncs::tg_line_memsize(self.inner) }
    }

    pub fn rect(&self) -> Rect {
        unsafe { LineFuncs::tg_line_rect(self.inner) }.into()
    }

    pub fn num_points(&self) -> usize {
        unsafe { LineFuncs::tg_line_num_points(self.inner) }
            .try_into()
            .unwrap()
    }

    pub fn points(&self) -> LinePoints<'_> {
        let points = unsafe { LineFuncs::tg_line_points(self.inner) };
        let len = self.num_points();
        LinePoints {
            points,
            front_index: 0,
            back_index: len,
            _lifetime: PhantomData,
        }
    }

    pub fn point(&self, index: usize) -> Option<Point> {
        (index < self.num_points()).then(|| unsafe {
            self.point_unchecked(index)
        })
    }

    /// # Safety
    ///
    /// The provided index must be less than the value returned by self.num_points()
    pub unsafe fn point_unchecked(&self, index: usize) -> Point {
        unsafe {
            LineFuncs::tg_line_point_at(self.inner, index.try_into().unwrap_unchecked() ).into()
        }
    }

    pub fn num_segments(&self) -> usize {
        unsafe { LineFuncs::tg_line_num_segments(self.inner) }
            .try_into()
            .unwrap()
    }

    pub fn segment(&self, index: usize) -> Option<Segment> {
        (index < self.num_segments()).then(|| unsafe {
            self.segment_unchecked(index)
        })
    }

    /// # Safety
    ///
    /// The provided index must be less than the value returned by self.num_segments()
    pub unsafe fn segment_unchecked(&self, index: usize) -> Segment {
        unsafe {
            LineFuncs::tg_line_segment_at(self.inner, index.try_into().unwrap_unchecked() ).into()
        }

    }
    pub fn clockwise(&self) -> bool {
        unsafe { LineFuncs::tg_line_clockwise(self.inner) }
    }

    pub fn xxx(&self) {
        unsafe {
            LineFuncs::tg_line_clone;
            LineFuncs::tg_line_copy;
            // LineFuncs::tg_line_free;
            LineFuncs::tg_line_index_level_num_rects;
            LineFuncs::tg_line_index_level_rect;
            LineFuncs::tg_line_index_num_levels;
            LineFuncs::tg_line_length(self.inner);
            LineFuncs::tg_line_index_spread;
            LineFuncs::tg_line_line_search;
            LineFuncs::tg_line_memsize;
            LineFuncs::tg_line_nearest_segment;
            LineFuncs::tg_line_point_at;
            LineFuncs::tg_line_points;
            LineFuncs::tg_line_rect;
            LineFuncs::tg_line_segment_at;
        }
    }

    pub fn index_spread(&self) -> i32 {
        unsafe {
            LineFuncs::tg_line_index_spread(self.inner)
        }
    }

    pub fn index_num_levels(&self) -> usize {
        unsafe {
            LineFuncs::tg_line_index_num_levels(self.inner).try_into().unwrap()
        }
    }

    pub fn index_level_num_rects(&self, level_index: usize) -> usize {
        unsafe {
            LineFuncs::tg_line_index_level_num_rects(self.inner, level_index.try_into().unwrap()).try_into().unwrap()
        }
    }

    pub fn index_level_rect(&self, level_index: usize, rect_index: usize) -> Option<Rect> {
        if level_index < self.index_num_levels() && rect_index < self.index_level_num_rects(level_index) {
           Some(unsafe { self.index_level_rect_unchecked(level_index, rect_index) })
        } else {
            None
        }
    }

    /// # Safety
    ///
    /// * level_index must be less than the value returned by self.index_num_levels()
    /// * rect_index must be less than the value returned by self.index_level_num_rects()
    pub unsafe fn index_level_rect_unchecked(&self, level_index: usize, rect_index: usize) -> Rect {
        // Safety: tg performs bounds checking on the provided indexes
        unsafe {
            LineFuncs::tg_line_index_level_rect(self.inner, level_index.try_into().unwrap_unchecked(), rect_index.try_into().unwrap_unchecked())
        }.into()
    }

    /// The length of the whole line (the sum of the lengths of its segments)
    pub fn length(&self) -> f64 {
        unsafe { LineFuncs::tg_line_length(self.inner) }
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

pub struct LinePoints<'a> {
    points: *const tg_point,
    front_index: usize,
    back_index: usize,
    _lifetime: PhantomData<&'a Line>,
}

impl<'a> Iterator for LinePoints<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let value = (self.front_index < self.back_index)
            .then(|| unsafe { self.points.add(self.front_index).read() }.into());
        self.front_index += 1;
        value
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.back_index - self.front_index;
        (size, Some(size))
    }
}

impl<'a> ExactSizeIterator for LinePoints<'a> {
    fn len(&self) -> usize {
        self.back_index - self.front_index
    }
}

impl<'a> DoubleEndedIterator for LinePoints<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let value =
        (self.front_index < self.back_index)
            .then(|| unsafe { self.points.add(self.back_index - 1).read() }.into());
        self.back_index -= 1;
        value
    }
}
