//! Polygon type and associated code
//!
//! - [ ] Create type
//! - [ ] Make Send + Sync
//! - [ ] impl Drop
//! - [ ] Add GeometryConstructors
//! - [ ] Add GeometryConstructorsEx
//! - [ ] Add GeometryParsing
//! - [ ] Add GeometryAccessors
//! - [ ] Add GeometryWriting
//! - [ ] Add GeometryPredicates
//! - [ ] Add tg_sys conversions
//! - [ ] Add PolFuncs
//! - [ ] Standard traits
//! - [ ] Documentation

use std::{
    alloc::{handle_alloc_error, Layout},
    ptr::NonNull,
};

use tg_sys::{tg_geom, GeometryConstructors};

#[derive(Debug)]
pub struct Geom {
    inner: NonNull<tg_geom>,
}

// GeometryConstructors
impl Geom {
    pub fn as_raw(&self) -> *mut tg_geom {
        self.inner.as_ptr()
    }

    pub fn from_raw(raw: *mut tg_geom) -> Option<Geom> {
        Some(Geom {
            inner: NonNull::new(raw)?,
        })
    }

    /// # Safety
    ///
    /// raw must point to a valid [`tg_geom`]`
    pub unsafe fn from_raw_unchecked(raw: *mut tg_geom) -> Geom {
        Geom {
            inner: unsafe { NonNull::new_unchecked(raw) },
        }
    }

    /// Create a new geometry from the current one by performing a deep copy.
    ///
    /// The tg C library calls this `tg_geom_copy`, but the semantics don't
    /// match rust expectations for the term "copy", so we call it "duplicate"
    /// instead.
    pub fn duplicate(&self) -> Geom {
        let raw = unsafe { GeometryConstructors::tg_geom_copy(self.as_raw()) };
        if raw.is_null() {
            Geom::handle_alloc_error();
        }
        Geom {
            inner: unsafe { NonNull::new_unchecked(raw) },
        }
    }

    pub(crate) fn handle_alloc_error() -> ! {
        let layout = Layout::new::<tg_geom>();
        handle_alloc_error(layout);
    }
}

impl Clone for Geom {
    fn clone(&self) -> Self {
        let raw = unsafe { GeometryConstructors::tg_geom_clone(self.as_raw()) };
        if raw.is_null() {
            Geom::handle_alloc_error();
        }
        unsafe { Geom::from_raw_unchecked(raw) }
    }
}

impl Drop for Geom {
    fn drop(&mut self) {
        unsafe {
            GeometryConstructors::tg_geom_free(self.as_raw());
        }
    }
}

impl From<*mut tg_geom> for Geom {
    #[warn(clippy::not_unsafe_ptr_arg_deref)]
    fn from(value: *mut tg_geom) -> Self {
        if value.is_null() {
            Geom::handle_alloc_error()
        }
        unsafe { Geom::from_raw_unchecked(value) } // false clippy alarm?
    }
}
