//! Line type and associated code
//!
//! - [x] Create type and constructors
//! - [x] Make Send + Sync
//! - [x] impl Drop
//! - [x] Create accessors
//! - [x] Add tg_sys conversions
//! - [ ] Add Geom conversions
//! - [x] Add LineFuncs
//! - [x] Add LineFuncs "iterators"
//! - [ ] Standard traits
//! - [ ] Serde traits
//! - [ ] Documentation

use std::{
    alloc::{handle_alloc_error, Layout},
    fmt,
    process::abort,
    ptr::NonNull,
};

use tg_sys::{tg_line, tg_point, tg_rect, tg_segment, GeometryConstructors, LineFuncs};

use crate::{Geom, IndexType, NearestSegmentVisitor, Point, Rect, SearchVisitor, Segment};

pub struct Line {
    inner: NonNull<tg_line>,
}

/// Constructors
impl Line {
    pub fn new(points: &[Point]) -> Line {
        let ptr = points.as_ptr() as *const tg_point;
        let len = points.len().try_into().unwrap();
        let raw = unsafe { LineFuncs::tg_line_new(ptr, len) };
        if raw.is_null() {
            Self::handle_alloc_error()
        }
        unsafe { Self::from_raw_unchecked(raw) }
    }

    pub fn new_indexed(points: &[Point], index: IndexType) -> Line {
        let ptr = points.as_ptr() as *const tg_point;
        let len = points.len().try_into().unwrap();
        let raw = unsafe { LineFuncs::tg_line_new_ix(ptr, len, index.into()) };
        if raw.is_null() {
            Self::handle_alloc_error();
        }
        unsafe { Line::from_raw_unchecked(raw) }
    }

    pub fn as_raw(&self) -> *mut tg_line {
        self.inner.as_ptr()
    }

    pub fn from_raw(raw: *mut tg_line) -> Option<Line> {
        Some(Line {
            inner: NonNull::new(raw)?,
        })
    }

    /// # Safety
    ///
    /// `raw` must be non-null
    pub unsafe fn from_raw_unchecked(raw: *mut tg_line) -> Line {
        Line {
            inner: unsafe { NonNull::new_unchecked(raw) },
        }
    }

    pub fn duplicate(&self) -> Line {
        let raw = unsafe { LineFuncs::tg_line_copy(self.as_raw()) };
        if raw.is_null() {
            Line::handle_alloc_error();
        }
        unsafe { Line::from_raw_unchecked(raw) }
    }

    pub fn geom(&self) -> Geom {
        let raw = unsafe { GeometryConstructors::tg_geom_new_linestring(self.as_raw()) };
        if raw.is_null() {
            Geom::handle_alloc_error();
        }
        unsafe { Geom::from_raw_unchecked(raw) }
    }

    fn handle_alloc_error() -> ! {
        let layout = Layout::new::<tg_line>();
        handle_alloc_error(layout)
    }
}

/// Operations from LineFuncs
impl Line {
    pub fn memsize(&self) -> usize {
        unsafe { LineFuncs::tg_line_memsize(self.as_raw()) }
    }

    pub fn rect(&self) -> Rect {
        unsafe { LineFuncs::tg_line_rect(self.as_raw()) }.into()
    }

    pub fn num_points(&self) -> usize {
        unsafe { LineFuncs::tg_line_num_points(self.as_raw()) }
            .try_into()
            .unwrap()
    }

    pub fn is_empty(&self) -> bool {
        self.num_points() == 0
    }

    pub fn points(&self) -> &[Point] {
        unsafe {
            std::slice::from_raw_parts(
                LineFuncs::tg_line_points(self.as_raw()) as *const Point,
                self.num_points(),
            )
        }
    }

    pub fn point(&self, index: usize) -> Option<Point> {
        (index < self.num_points()).then(|| self.point_unchecked(index))
    }

    /// Get the point at the given index.
    ///
    /// Returns an empty point if index is out of bounds
    pub fn point_unchecked(&self, index: usize) -> Point {
        unsafe {
            LineFuncs::tg_line_point_at(self.as_raw(), index.try_into().unwrap_unchecked()).into()
        }
    }

    pub fn num_segments(&self) -> usize {
        unsafe { LineFuncs::tg_line_num_segments(self.as_raw()) }
            .try_into()
            .unwrap()
    }

    pub fn segment(&self, index: usize) -> Option<Segment> {
        (index < self.num_segments()).then(|| unsafe { self.segment_unchecked(index) })
    }

    /// # Safety
    ///
    /// The provided index must be less than the value returned by self.num_segments()
    pub unsafe fn segment_unchecked(&self, index: usize) -> Segment {
        unsafe {
            LineFuncs::tg_line_segment_at(self.as_raw(), index.try_into().unwrap_unchecked()).into()
        }
    }
    pub fn clockwise(&self) -> bool {
        unsafe { LineFuncs::tg_line_clockwise(self.as_raw()) }
    }

    pub fn index_spread(&self) -> usize {
        unsafe {
            LineFuncs::tg_line_index_spread(self.as_raw())
                .try_into()
                .unwrap()
        }
    }

    pub fn index_num_levels(&self) -> usize {
        unsafe {
            LineFuncs::tg_line_index_num_levels(self.as_raw())
                .try_into()
                .unwrap()
        }
    }

    pub fn index_level_num_rects(&self, level_index: usize) -> usize {
        unsafe {
            LineFuncs::tg_line_index_level_num_rects(self.as_raw(), level_index.try_into().unwrap())
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
            LineFuncs::tg_line_index_level_rect(
                self.as_raw(),
                level_index.try_into().unwrap_unchecked(),
                rect_index.try_into().unwrap_unchecked(),
            )
        }
        .into()
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
            LineFuncs::tg_line_line_search(
                self.as_raw(),
                other.as_raw(),
                visit::<V>,
                visitor as *mut V as *mut libc::c_void,
            )
        }
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
            LineFuncs::tg_line_nearest_segment(
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

    #[allow(unused_variables, unused_mut, clippy::let_and_return)]
    pub fn simple_nearest_segment(&self, point: Point) -> Vec<Segment> {
        let mut vec = Vec::with_capacity(self.num_segments());
        // self.nearest_segment((
        //     |seg: Segment, more| seg.distance(point)

        // ));
        vec
    }

    /// The length of the whole line (the sum of the lengths of its segments)
    pub fn length(&self) -> f64 {
        unsafe { LineFuncs::tg_line_length(self.as_raw()) }
    }
}

#[cfg(feature = "atomics")]
unsafe impl Send for Line {}
#[cfg(feature = "atomics")]
unsafe impl Sync for Line {}

impl Clone for Line {
    fn clone(&self) -> Line {
        let raw = unsafe { LineFuncs::tg_line_clone(self.as_raw()) };
        if raw.is_null() {
            Line::handle_alloc_error();
        }
        unsafe { Line::from_raw_unchecked(raw) }
    }
}
impl Default for Line {
    fn default() -> Self {
        Line::new(&[])
    }
}

impl fmt::Debug for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut tuple = f.debug_tuple("Line");
        for point in self.points() {
            tuple.field(&point);
        }
        tuple.finish()
    }
}
impl Drop for Line {
    fn drop(&mut self) {
        unsafe { LineFuncs::tg_line_free(self.as_raw()) }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::identity;

    use super::Line;
    use crate::{Point, Rect, SearchVisitor, Segment};

    #[test]
    fn line_search() {
        let l1 = Line::new(&[
            Point::new(-1., -1.),
            Point::new(0., 0.),
            Point::new(1., 0.),
            Point::new(2., -1.),
        ]);
        let l2 = Line::new(&[
            Point::new(-5., -0.5),
            Point::new(5., -0.5),
            Point::new(5., -0.25),
            Point::new(-5., -0.25),
        ]);

        let mut intersections = Vec::new();
        let mut intersection_visitor = |seg1: Segment, idx1, seg2, idx2| {
            eprintln!("{seg1:?}:{idx1} {seg2:?}:{idx2}");
            intersections.push(seg1.intersects_segment(seg2));
            true
        };
        l1.line_search(&l2, &mut intersection_visitor);
        assert!(intersections.iter().copied().all(identity));
        assert_eq!(intersections.len(), 4);
    }

    #[test]
    fn line_search_return_value() {
        let l1 = Line::new(&[
            Point::new(-1., -1.),
            Point::new(0., 0.),
            Point::new(1., 0.),
            Point::new(2., -1.),
        ]);
        let l2 = Line::new(&[
            Point::new(-5., -0.5),
            Point::new(5., -0.5),
            Point::new(5., -0.25),
            Point::new(-5., -0.25),
        ]);
        let mut true_ct = 0;
        let mut true_visitor = |_, _, _, _| {
            true_ct += 1;
            true
        };
        l1.line_search(&l2, &mut true_visitor);
        let mut false_ct = 0;
        let mut false_visitor = |_, _, _, _| {
            false_ct += 1;
            false
        };
        l1.line_search(&l2, &mut false_visitor);
        assert_eq!(true_ct, 4);
        assert_eq!(false_ct, 1);
    }

    #[test]
    fn line_search_visitor() {
        struct Visitor {
            ct: usize,
        }
        impl SearchVisitor for Visitor {
            fn visit(&mut self, _: Segment, _: usize, _: Segment, _: usize) -> bool {
                self.ct += 1;
                true
            }
        }
        let l1 = Line::new(&[
            Point::new(-1., -1.),
            Point::new(0., 0.),
            Point::new(1., 0.),
            Point::new(2., -1.),
        ]);
        let l2 = Line::new(&[
            Point::new(-5., -0.5),
            Point::new(5., -0.5),
            Point::new(5., -0.25),
            Point::new(-5., -0.25),
        ]);
        let mut visitor = Visitor { ct: 0 };
        l1.line_search(&l2, &mut visitor);
        assert_eq!(visitor.ct, 4);
    }

    #[test]
    fn points() {
        let line = Line::new(&[
            Point::new(-1., -1.),
            Point::new(0., 0.),
            Point::new(1., 0.),
            Point::new(2., -1.),
        ]);

        let points = line.points();
        assert_eq!(points.get(0), Some(&Point::new(-1., -1.)));
        assert_eq!(points.get(1), Some(&Point::new(0., 0.)));
        assert_eq!(points.get(2), Some(&Point::new(1., 0.)));
        assert_eq!(points.get(3), Some(&Point::new(2., -1.)));
        assert_eq!(points.get(4), None);
    }

    #[test]
    fn nearest_segment() {
        let l1 = Line::new(&[
            Point::new(-1., -1.),
            Point::new(0., 0.),
            Point::new(1., 0.),
            Point::new(2., -1.),
        ]);

        let mut ct = 0;
        l1.nearest_segment(&mut (
            |seg: Segment, more: &mut i32| {
                eprintln!("segment_distance:{seg:?}:{more}");
                seg.a().x()
            },
            |rect: Rect, more: &mut i32| {
                eprintln!("rectangle_distance:{rect:?}:{more}");
                rect.min().x()
            },
            |seg: Segment, distance, index| {
                eprintln!("visit:{seg:?}:{distance}:{index}");
                ct += 1;
                true
            },
        ));
        assert_eq!(ct, 0);
    }
}
