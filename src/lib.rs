//! This crate provides reference-counted slices that support easy subdivision.

#![no_std]
#![deny(unsafe_code)]

extern crate alloc;

use alloc::rc::Rc;
use alloc::sync::Arc;
use core::ops::Deref;

/// A read-only view into an underlying reference-counted slice.
///
/// The associated functions provided for this type do not take a receiver to avoid conflicting
/// with (present or future) methods on `[T]`, since `RcSlice<T>: Deref<Target = [T]>`.
pub struct RcSlice<T> {
    underlying: Rc<[T]>,
    start: usize,
    end: usize,
}

impl<T> Clone for RcSlice<T> {
    fn clone(&self) -> Self {
        Self {
            underlying: self.underlying.clone(),
            start: self.start,
            end: self.end,
        }
    }
}

impl<T> AsRef<[T]> for RcSlice<T> {
    fn as_ref(&self) -> &[T] {
        &self.underlying[self.start..self.end]
    }
}

impl<T> Deref for RcSlice<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T> From<Rc<[T]>> for RcSlice<T> {
    fn from(underlying: Rc<[T]>) -> Self {
        let end = underlying.len();

        Self {
            underlying,
            start: 0,
            end,
        }
    }
}

impl<T> RcSlice<T> {
    /// Returns the starting and ending indices of the view `it` within the underlying slice.
    pub fn bounds(it: &Self) -> (usize, usize) {
        (it.start, it.end)
    }

    /// Increases the starting index of `it` by `incr` places, and returns a reference to the
    /// elements cut off by this operation.
    ///
    /// Returns `None` and leaves `it` unchanged if this operation would make the starting index
    /// greater than the ending index.
    pub fn advance(it: &mut Self, incr: usize) -> Option<&[T]> {
        let cut = it.start.checked_add(incr)?;

        if cut <= it.end {
            let shed = &it.underlying[it.start..cut];
            it.start = cut;

            Some(shed)
        } else {
            None
        }
    }

    /// Mutates the view `it` to point to only the first `index` elements of the underlying slice,
    /// and returns a new view of the remaining elements.
    ///
    /// Returns `None` and leaves `it` unchanged if the underlying slice has fewer than `index`
    /// elements.
    pub fn split_off_before(it: &mut Self, index: usize) -> Option<Self> {
        let cut = it.start.checked_add(index)?;

        if cut <= it.end {
            let mut front = it.clone();
            front.end = cut;
            it.start = cut;

            Some(front)
        } else {
            None
        }
    }
}

/// A read-only view into an underlying atomically reference-counted slice.
///
/// The associated functions provided for this type do not take a receiver to avoid conflicting
/// with (present or future) methods on `[T]`, since `ArcSlice<T>: Deref<Target = [T]>`.
pub struct ArcSlice<T> {
    underlying: Arc<[T]>,
    start: usize,
    end: usize,
}

impl<T> Clone for ArcSlice<T> {
    fn clone(&self) -> Self {
        Self {
            underlying: self.underlying.clone(),
            start: self.start,
            end: self.end,
        }
    }
}

impl<T> AsRef<[T]> for ArcSlice<T> {
    fn as_ref(&self) -> &[T] {
        &self.underlying[self.start..self.end]
    }
}

impl<T> Deref for ArcSlice<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T> From<Arc<[T]>> for ArcSlice<T> {
    fn from(underlying: Arc<[T]>) -> Self {
        let end = underlying.len();

        Self {
            underlying,
            start: 0,
            end,
        }
    }
}

impl<T> ArcSlice<T> {
    /// Returns the starting and ending indices of the view `it` within the underlying slice.
    pub fn bounds(it: &Self) -> (usize, usize) {
        (it.start, it.end)
    }

    /// Increases the starting index of `it` by `incr` places, and returns a reference to the
    /// elements cut off by this operation.
    ///
    /// Returns `None` and leaves `it` unchanged if this operation would make the starting index
    /// greater than the ending index.
    pub fn advance(it: &mut Self, incr: usize) -> Option<&[T]> {
        let cut = it.start.checked_add(incr)?;

        if cut <= it.end {
            let shed = &it.underlying[it.start..cut];
            it.start = cut;

            Some(shed)
        } else {
            None
        }
    }

    /// Mutates the view `it` to point to only the first `index` elements of the underlying slice,
    /// and returns a new view of the remaining elements.
    ///
    /// Returns `None` and leaves `it` unchanged if the underlying slice has fewer than `index`
    /// elements.
    pub fn split_off_before(it: &mut Self, index: usize) -> Option<Self> {
        let cut = it.start.checked_add(index)?;

        if cut <= it.end {
            let mut front = it.clone();
            front.end = cut;
            it.start = cut;

            Some(front)
        } else {
            None
        }
    }
}

pub type RcBytes = RcSlice<u8>;

pub type ArcBytes = ArcSlice<u8>;
