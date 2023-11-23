//! Ring type and associated code
//!
//! - [x] Create type and constructors
//! - [x] Make Send + Sync
//! - [x] impl Drop
//! - [x] Create accessors
//! - [x] Add tg_sys conversions
//! - [ ] Add Geom conversions
//! - [x] Add RingFuncs
//! - [ ] Standard traits
//! - [ ] Decide if PartialEq should include chirality or origin
//! - [ ] Documentation

use std::{
    alloc::{handle_alloc_error, Layout},
    fmt,
    process::abort,
    ptr::NonNull,
};

use tg_sys::{tg_point, tg_rect, tg_ring, tg_segment, RingFuncs};

use crate::{IndexType, SearchVisitor, NearestSegmentVisitor, Point, Rect, Segment, Line};

pub struct Ring {
    inner: NonNull<tg_ring>,
}

/// Constructors
impl Ring {
    pub fn new(points: &[Point]) -> Ring {
        let len = points.len().try_into().expect("len must be a valid i32");
        let ptr = points.as_ptr() as *const tg_point;
        let raw = unsafe { RingFuncs::tg_ring_new(ptr, len) };
        if raw.is_null() {
            Ring::handle_alloc_error();
        }
        unsafe { Ring::from_raw_unchecked(raw) }
    }

    pub fn new_indexed(points: &[Point], index: IndexType) -> Ring {
        let ptr = points.as_ptr() as *const tg_point;
        let len = points.len().try_into().expect("len must be a valid i32");
        let raw = unsafe { RingFuncs::tg_ring_new_ix(ptr, len, index.into()) };
        if raw.is_null() {
            Ring::handle_alloc_error();
        }
        unsafe { Ring::from_raw_unchecked(raw) }
    }

    pub fn as_raw(&self) -> *mut tg_ring {
        self.inner.as_ptr()
    }

    pub fn from_raw(raw: *mut tg_ring) -> Option<Ring> {
        Some(Ring {
            inner: NonNull::new(raw)?,
        })
    }

    /// # Safety
    ///
    /// `raw` must point to a valid tg_ring.
    pub unsafe fn from_raw_unchecked(raw: *mut tg_ring) -> Ring {
        Ring {
            inner: NonNull::new_unchecked(raw),
        }
    }

    pub fn duplicate(&self) -> Ring {
        let raw = unsafe { RingFuncs::tg_ring_copy(self.as_raw()) };
        if raw.is_null() {
            Ring::handle_alloc_error();
        }
        unsafe { Ring::from_raw_unchecked(raw) }
    }

    fn handle_alloc_error() -> ! {
        let layout = Layout::new::<tg_ring>();
        handle_alloc_error(layout)
    }
}

/// Operations from RingFuncs
impl Ring {
    pub fn memsize(&self) -> usize {
        unsafe { RingFuncs::tg_ring_memsize(self.as_raw()) }
    }
    pub fn rect(&self) -> Rect {
        unsafe { RingFuncs::tg_ring_rect(self.as_raw()) }.into()
    }
    pub fn num_points(&self) -> usize {
        unsafe { RingFuncs::tg_ring_num_points(self.as_raw()) as usize }
    }

    pub fn points(&self) -> &[Point] {
        unsafe {
            std::slice::from_raw_parts(
                RingFuncs::tg_ring_points(self.as_raw()) as *const Point,
                self.num_points(),
            )
        }
    }

    /// Get the point at the given index.
    pub fn point(&self, index: usize) -> Option<Point> {
        (index < self.num_points()).then(|| self.point_unchecked(index))
    }

    /// Get the point at the given index.
    ///
    /// Returns an empty point if index is out of bounds
    pub fn point_unchecked(&self, index: usize) -> Point {
        unsafe {
            RingFuncs::tg_ring_point_at(
                self.as_raw(),
                index.try_into().expect("index is a valid c_int"),
            )
        }
        .into()
    }

    pub fn num_segments(&self) -> usize {
        unsafe { RingFuncs::tg_ring_num_segments(self.as_raw()) }
            .try_into()
            .expect("tg_ring_num_segments should return a valid usize")
    }

    pub fn segment(&self, index: usize) -> Option<Segment> {
        (index < self.num_segments()).then(|| self.segment_unchecked(index))
    }

    pub fn segment_unchecked(&self, index: usize) -> Segment {
        unsafe {
            RingFuncs::tg_ring_segment_at(
                self.as_raw(),
                index.try_into().expect("index is a valid c_int"),
            )
        }
        .into()
    }

    pub fn convex(&self) -> bool {
        unsafe { RingFuncs::tg_ring_convex(self.as_raw()) }
    }

    pub fn clockwise(&self) -> bool {
        unsafe { RingFuncs::tg_ring_clockwise(self.as_raw()) }
    }

    pub fn index_spread(&self) -> usize {
        unsafe {
            RingFuncs::tg_ring_index_spread(self.as_raw())
                .try_into()
                .unwrap()
        }
    }

    pub fn index_num_levels(&self) -> usize {
        unsafe {
            RingFuncs::tg_ring_index_num_levels(self.as_raw())
                .try_into()
                .unwrap()
        }
    }

    pub fn index_level_num_rects(&self, level_index: usize) -> usize {
        unsafe {
            RingFuncs::tg_ring_index_level_num_rects(self.as_raw(), level_index.try_into().unwrap())
                .try_into()
                .unwrap()
        }
    }

    pub fn index_level_rect(&self, level_index: usize, rect_index: usize) -> Option<Rect> {
        if level_index < self.index_num_levels()
            && rect_index < self.index_level_num_rects(level_index)
        {
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
            RingFuncs::tg_ring_index_level_rect(
                self.as_raw(),
                level_index.try_into().unwrap_unchecked(),
                rect_index.try_into().unwrap_unchecked(),
            )
        }
        .into()
    }

    pub fn nearest_segment<V: NearestSegmentVisitor>(&self, visitor: &mut V) {
        extern "C" fn seg_dist<V: NearestSegmentVisitor>(
            segment: tg_segment,
            more: *mut libc::c_int,
            udata: *mut libc::c_void,
        ) -> libc::c_double {
            let visitor = unsafe { (udata as *mut V).as_mut() }.unwrap();
            let more = unsafe { more.as_mut() }.unwrap();
            visitor.segment_distance(segment.into(), more)
        }

        extern "C" fn rect_dist<V: NearestSegmentVisitor>(
            rect: tg_rect,
            more: *mut libc::c_int,
            udata: *mut libc::c_void,
        ) -> libc::c_double {
            let visitor = unsafe { (udata as *mut V).as_mut() }.unwrap();
            let more = unsafe { more.as_mut() }.unwrap();
            visitor.rect_distance(rect.into(), more)
        }

        extern "C" fn visit<V: NearestSegmentVisitor>(
            segment: tg_segment,
            distance: libc::c_double,
            index: libc::c_int,
            udata: *mut libc::c_void,
        ) -> bool {
            let visitor = unsafe { (udata as *mut V).as_mut() }.unwrap();
            visitor.visit(segment.into(), distance, index.try_into().unwrap())
        }

        let ok = unsafe {
            RingFuncs::tg_ring_nearest_segment(
                self.as_raw(),
                rect_dist::<V>,
                seg_dist::<V>,
                visit::<V>,
                visitor as *mut V as *mut libc::c_void,
            )
        };
        if !ok {
            // out of memory
            abort()
        }
    }

    pub fn line_search<V: SearchVisitor>(&self, other: &Line, visitor: &mut V) {
        extern "C" fn visit<V: SearchVisitor>(
            a_seg: tg_segment,
            a_idx: libc::c_int,
            b_seg: tg_segment,
            b_idx: libc::c_int,
            udata: *mut libc::c_void,
        ) -> bool {
            let visitor = unsafe { (udata as *mut V).as_mut() }.unwrap();
            visitor.visit(
                a_seg.into(),
                a_idx.try_into().unwrap(),
                b_seg.into(),
                b_idx.try_into().unwrap(),
            )
        }
        unsafe {
            RingFuncs::tg_ring_line_search(
                self.as_raw(),
                other.as_raw(),
                visit::<V>,
                visitor as *mut V as *mut libc::c_void,
            )
        }
    }

    pub fn ring_search<V: SearchVisitor>(&self, other: &Ring, visitor: &mut V) {
        extern "C" fn visit<V: SearchVisitor>(
            a_seg: tg_segment,
            a_idx: libc::c_int,
            b_seg: tg_segment,
            b_idx: libc::c_int,
            udata: *mut libc::c_void,
        ) -> bool {
            let visitor = unsafe { (udata as *mut V).as_mut() }.unwrap();
            visitor.visit(
                a_seg.into(),
                a_idx.try_into().unwrap(),
                b_seg.into(),
                b_idx.try_into().unwrap(),
            )
        }
        unsafe {
            RingFuncs::tg_ring_ring_search(
                self.as_raw(),
                other.as_raw(),
                visit::<V>,
                visitor as *mut V as *mut libc::c_void,
            )
        }
    }

    pub fn area(&self) -> f64 {
        unsafe { RingFuncs::tg_ring_area(self.as_raw())}
    }

    pub fn perimeter(&self) -> f64 {
        unsafe { RingFuncs::tg_ring_perimeter(self.as_raw())}
    }
}

#[cfg(feature = "atomics")]
unsafe impl Send for Ring {}
#[cfg(feature = "atomics")]
unsafe impl Sync for Ring {}

impl Drop for Ring {
    fn drop(&mut self) {
        unsafe {
            RingFuncs::tg_ring_free(self.as_raw());
        }
    }
}

impl Clone for Ring {
    fn clone(&self) -> Self {
        let new = unsafe { RingFuncs::tg_ring_clone(self.as_raw()) };
        if new.is_null() {
            Ring::handle_alloc_error();
        }
        unsafe { Ring::from_raw_unchecked(new) }
    }
}

impl fmt::Debug for Ring {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Ring").field("inner", &self.inner).finish()
    }
}
impl PartialEq for Ring {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

#[cfg(test)]
mod tests {
    use super::Ring;
    use crate::Point;
    #[test]
    fn clones_and_duplicates_are_equal() {
        let ring = Ring::new(&[
            Point::new(0., 1.),
            Point::new(14., -22.5),
            Point::new(0., 0.),
        ]);

        assert_eq!(ring, ring);
        assert_eq!(ring, ring.clone());
        assert_eq!(ring, ring.duplicate());
        assert_eq!(ring.clone(), ring.duplicate());
    }
}
