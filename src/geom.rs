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
use tg_sys::{tg_geom, GeometryConstructors};

#[derive(Debug)]
pub struct Geom {
    inner: *mut tg_geom,
}

// GeometryConstructors
impl Geom {
    /// Create a new geometry from the current one by performing a deep copy.
    ///
    /// The tg C library calls this `tg_geom_copy`, but the semantics don't
    /// match rust expectations for the term "copy", so we call it "duplicate"
    /// instead.
    pub fn duplicate(&self) -> Self {
        Self {
            inner: unsafe { GeometryConstructors::tg_geom_copy(self.inner) },
        }
    }
}

impl Clone for Geom {
    fn clone(&self) -> Self {
        Self {
            inner: unsafe { GeometryConstructors::tg_geom_clone(self.inner) },
        }
    }
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
