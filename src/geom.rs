use std::alloc::{Layout, handle_alloc_error};

use tg_sys::{tg_geom, GeometryConstructors::{tg_geom_new_point, tg_geom_free}};
use crate::Point;

#[derive(Debug)]
pub struct Geom {
inner: *mut tg_geom,
}

impl From<Point> for Geom {
fn from(value: Point) -> Self {
    let geom = unsafe { tg_geom_new_point(value.into()) };
    if geom.is_null() {
        let layout = Layout::new::<tg_geom>();
        handle_alloc_error(layout);
    }
    Geom {
        inner: geom,
    }
}
}

impl Drop for Geom {
fn drop(&mut self) {
    unsafe {
        tg_geom_free(self.inner);
    }
}
}

