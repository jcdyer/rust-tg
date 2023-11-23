use crate::{Segment, Rect};

pub trait NearestSegmentVisitor {
    fn segment_distance(&mut self, segment: Segment, more: &mut i32) -> f64;
    fn rect_distance(&mut self, rect: Rect, more: &mut i32) -> f64;
    fn visit(&mut self, segment: Segment, distance: f64, index: usize) -> bool;
}

impl<F1, F2, F3> NearestSegmentVisitor for (F1, F2, F3)
where
    for<'a> F1: FnMut(Segment, &'a mut i32) -> f64,
    for<'b> F2: FnMut(Rect, &'b mut i32) -> f64,
    F3: FnMut(Segment, f64, usize) -> bool,
{
    fn segment_distance(&mut self, segment: Segment, more: &mut i32) -> f64 {
        self.0(segment, more)
    }

    fn rect_distance(&mut self, rect: Rect, more: &mut i32) -> f64 {
        self.1(rect, more)
    }

    fn visit(&mut self, segment: Segment, distance: f64, index: usize) -> bool {
        self.2(segment, distance, index)
    }
}

pub trait SearchVisitor {
    fn visit(&mut self, a_seg: Segment, a_idx: usize, b_seg: Segment, b_idx: usize) -> bool;
}

impl<F> SearchVisitor for F
where
    F: FnMut(Segment, usize, Segment, usize) -> bool,
{
    fn visit(&mut self, a_seg: Segment, a_idx: usize, b_seg: Segment, b_idx: usize) -> bool {
        self(a_seg, a_idx, b_seg, b_idx)
    }
}
