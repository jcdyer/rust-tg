//! Polygon type and associated code
//!
//! - [ ] Create type
//! - [ ] Make Send + Sync
//! - [ ] impl Drop
//! - [ ] Add GeometryConstructors
//! - [ ] Add GeometryParsing
//! - [ ] Add GeometryAccessors
//! - [ ] Add GeometryWriting
//! - [ ] Add GeometryPredicates
//! - [ ] Add tg_sys conversions
//! - [ ] Add PolFuncs
//! - [ ] Standard traits
//! - [ ] Documentation

use std::alloc::{handle_alloc_error, Layout};

use crate::Point;
use tg_sys::{
    tg_geom,
    GeometryConstructors,
};

#[derive(Debug)]
pub struct Geom {
    inner: *mut tg_geom,
}

impl From<Point> for Geom {
    fn from(value: Point) -> Self {
        let geom = unsafe { GeometryConstructors::tg_geom_new_point(value.into()) };
        if geom.is_null() {
            let layout = Layout::new::<tg_geom>();
            handle_alloc_error(layout);
        }
        Geom { inner: geom }
    }
}

impl Drop for Geom {
    fn drop(&mut self) {
        unsafe {
            GeometryConstructors::tg_geom_free(self.inner);
        }
    }
}
