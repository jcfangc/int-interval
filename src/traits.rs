use core::num::ParseIntError;

use crate::{OneTwo, ZeroOneTwo};

pub(crate) mod forwarding;
pub(crate) use forwarding::impl_co_forwarding;

mod sealed;

/// Built-in integer coordinate type accepted by closed-open intervals.
pub trait IntPrimitive: sealed::Int {
    fn zero() -> Self;
    fn one() -> Self;

    fn min_value() -> Self;
    fn max_value() -> Self;

    fn as_f32(self) -> f32;
    fn as_f64(self) -> f64;

    fn checked_add(self, rhs: Self) -> Option<Self>;
    fn checked_sub(self, rhs: Self) -> Option<Self>;
    fn checked_mul(self, rhs: Self) -> Option<Self>;
    fn checked_div(self, rhs: Self) -> Option<Self>;
    fn checked_rem(self, rhs: Self) -> Option<Self>;

    fn saturating_add(self, rhs: Self) -> Self;
    fn saturating_sub(self, rhs: Self) -> Self;
    fn saturating_mul(self, rhs: Self) -> Self;

    fn checked_next(self) -> Option<Self>;

    fn parse_decimal(src: &str) -> Result<Self, ParseIntError>;
}

impl<T> IntPrimitive for T
where
    T: sealed::Int,
{
    #[inline]
    fn zero() -> Self {
        sealed::Int::zero()
    }

    #[inline]
    fn one() -> Self {
        sealed::Int::one()
    }

    #[inline]
    fn min_value() -> Self {
        sealed::Int::min_value()
    }

    #[inline]
    fn max_value() -> Self {
        sealed::Int::max_value()
    }

    #[inline]
    fn as_f32(self) -> f32 {
        sealed::Int::as_f32(self)
    }

    #[inline]
    fn as_f64(self) -> f64 {
        sealed::Int::as_f64(self)
    }

    #[inline]
    fn checked_add(self, rhs: Self) -> Option<Self> {
        sealed::Int::checked_add(self, rhs)
    }

    #[inline]
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        sealed::Int::checked_sub(self, rhs)
    }

    #[inline]
    fn checked_mul(self, rhs: Self) -> Option<Self> {
        sealed::Int::checked_mul(self, rhs)
    }

    #[inline]
    fn checked_div(self, rhs: Self) -> Option<Self> {
        sealed::Int::checked_div(self, rhs)
    }

    #[inline]
    fn checked_rem(self, rhs: Self) -> Option<Self> {
        sealed::Int::checked_rem(self, rhs)
    }

    #[inline]
    fn saturating_add(self, rhs: Self) -> Self {
        sealed::Int::saturating_add(self, rhs)
    }

    #[inline]
    fn saturating_sub(self, rhs: Self) -> Self {
        sealed::Int::saturating_sub(self, rhs)
    }

    #[inline]
    fn saturating_mul(self, rhs: Self) -> Self {
        sealed::Int::saturating_mul(self, rhs)
    }

    #[inline]
    fn checked_next(self) -> Option<Self> {
        sealed::Int::checked_next(self)
    }

    #[inline]
    fn parse_decimal(src: &str) -> Result<Self, ParseIntError> {
        sealed::Int::parse_decimal(src)
    }
}

/// Built-in unsigned integer type used for exact interval measures.
pub trait UnsignedPrimitive: IntPrimitive + sealed::Unsigned {}

impl<T> UnsignedPrimitive for T where T: IntPrimitive + sealed::Unsigned {}

/// Primitive types associated with a closed-open integer interval.
pub trait COPrimitive {
    type CoordType: IntPrimitive;
    type MeasureType: UnsignedPrimitive;
}

/// Construction capability for a valid closed-open interval.
///
/// Implementations must preserve the invariant:
///
/// ```text
/// start < end_excl
/// ```
pub trait COConstruct: COPrimitive + Sized {
    /// Constructs `[start, end_excl)`, returning `None` for an empty or
    /// reversed interval.
    fn try_new(start: Self::CoordType, end_excl: Self::CoordType) -> Option<Self>;

    /// Constructs `[start, end_excl)` without checking the interval invariant.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that:
    ///
    /// ```text
    /// start < end_excl
    /// ```
    unsafe fn new_unchecked(start: Self::CoordType, end_excl: Self::CoordType) -> Self;
}

/// Construction capability based on a midpoint and an exact interval measure.
///
/// `len` is represented by the interval's exact unsigned measure type.
pub trait COMidpointConstruct: COConstruct {
    /// Constructs an interval centered around `mid` with exact length `len`.
    ///
    /// Returns `None` when `len` is zero or when the resulting bounds cannot
    /// be represented by `CoordType`.
    fn checked_from_midpoint_len(mid: Self::CoordType, len: Self::MeasureType) -> Option<Self>;

    /// Constructs an interval centered around `mid` with saturating endpoint
    /// arithmetic.
    ///
    /// Returns `None` when `len` is zero or saturation collapses the result
    /// into an empty interval.
    fn saturating_from_midpoint_len(mid: Self::CoordType, len: Self::MeasureType) -> Option<Self>;
}

/// Construction capability based on a start bound and an exact interval measure.
///
/// `len` is represented by the interval's exact unsigned measure type.
pub trait COStartLenConstruct: COConstruct {
    /// Constructs an interval starting at `start` with exact length `len`.
    ///
    /// The resulting interval is:
    ///
    /// ```text
    /// [start, start + len)
    /// ```
    ///
    /// Returns `None` when `len` is zero or when the resulting exclusive end
    /// bound cannot be represented by `CoordType`.
    fn checked_from_start_len(start: Self::CoordType, len: Self::MeasureType) -> Option<Self>;

    /// Constructs an interval starting at `start` with saturating endpoint
    /// arithmetic.
    ///
    /// The resulting interval starts at `start`, while the exclusive end bound
    /// is clamped to the maximum representable coordinate when needed.
    ///
    /// Returns `None` when `len` is zero or saturation collapses the result
    /// into an empty interval.
    fn saturating_from_start_len(start: Self::CoordType, len: Self::MeasureType) -> Option<Self>;
}

/// Boundary access capability for a closed-open interval.
pub trait COBounds: COPrimitive + Copy + Ord + Eq + core::fmt::Debug {
    /// Returns the inclusive lower bound.
    fn start(self) -> Self::CoordType;

    /// Returns the exclusive upper bound.
    fn end_excl(self) -> Self::CoordType;

    /// Returns the inclusive upper bound.
    ///
    /// This is the greatest coordinate contained in the interval.
    fn end_incl(self) -> Self::CoordType;
}

/// Containment and overlap predicates for closed-open intervals.
pub trait COPredicates: COBounds {
    /// Returns whether `x` is contained in this interval.
    fn contains(self, x: Self::CoordType) -> bool;

    /// Returns whether `other` is fully contained in this interval.
    fn contains_interval(self, other: Self) -> bool;

    /// Returns whether this interval and `other` overlap with positive length.
    fn intersects(self, other: Self) -> bool;

    /// Returns whether this interval and `other` touch at exactly one boundary
    /// without overlapping.
    fn is_adjacent(self, other: Self) -> bool;

    /// Returns whether this interval and `other` overlap or are adjacent.
    fn is_contiguous_with(self, other: Self) -> bool;
}

/// Range projection capability for a closed-open interval.
///
/// The returned range has the same half-open semantics as the interval:
///
/// ```text
/// [start, end_excl) -> start..end_excl
/// ```
pub trait CORange: COBounds + Sized {
    /// Returns the standard-library half-open range represented by this
    /// interval.
    fn to_range(self) -> core::ops::Range<Self::CoordType>;

    /// Returns the standard-library range used to iterate covered coordinates.
    ///
    /// This is equivalent to `self.to_range()`.
    #[inline]
    fn iter(self) -> core::ops::Range<Self::CoordType> {
        self.to_range()
    }
}

/// Algebraic operations for closed-open intervals.
pub trait COAlgebra: COConstruct + COBounds + COPredicates {
    /// Returns the overlapping region of two intervals, if any.
    fn intersection(self, other: Self) -> Option<Self>;

    /// Returns the smallest interval containing both intervals.
    fn convex_hull(self, other: Self) -> Self;

    /// Returns the interval strictly between two separated intervals.
    ///
    /// Returns `None` when the intervals overlap or are adjacent.
    fn between(self, other: Self) -> Option<Self>;

    /// Returns the union of two intervals.
    ///
    /// Contiguous intervals are merged into one interval; otherwise the two
    /// intervals are returned in ascending order.
    fn union(self, other: Self) -> OneTwo<Self>;

    /// Returns `self \ other`.
    ///
    /// The result may contain zero, one, or two residual intervals.
    fn difference(self, other: Self) -> ZeroOneTwo<Self>;

    /// Returns the symmetric difference of two intervals.
    ///
    /// The result contains points covered by exactly one operand and may
    /// contain zero, one, or two intervals.
    fn symmetric_difference(self, other: Self) -> ZeroOneTwo<Self>;
}

/// Exact measure capability for a closed-open interval.
pub trait COMeasure: COPrimitive {
    /// Returns the exact interval length.
    fn len(self) -> Self::MeasureType;
}

/// Representative-position capability for a closed-open interval.
pub trait COMidpoint: COPrimitive {
    /// Returns the midpoint coordinate, using floor rounding where required.
    fn midpoint(self) -> Self::CoordType;
}

/// Exact checked Minkowski operations whose images remain closed-open
/// integer intervals.
pub trait COCheckedMinkowskiLinear: COPrimitive + Sized {
    /// Returns the exact Minkowski sum `self + other`.
    fn checked_minkowski_add(self, other: Self) -> Option<Self>;

    /// Returns the exact Minkowski subtraction `self - other`.
    fn checked_minkowski_sub(self, other: Self) -> Option<Self>;

    /// Returns the exact translation `self + scalar`.
    fn checked_minkowski_add_scalar(self, scalar: Self::CoordType) -> Option<Self>;

    /// Returns the exact translation `self - scalar`.
    fn checked_minkowski_sub_scalar(self, scalar: Self::CoordType) -> Option<Self>;
}

/// Checked interval hulls of non-linear Minkowski images.
///
/// For discrete integer intervals, multiplication and division may produce
/// non-contiguous point sets. These methods return a containing interval hull,
/// not necessarily an exact image.
pub trait COCheckedMinkowskiHull: COPrimitive + Sized {
    /// Returns the interval hull containing every point in `self * other`.
    fn checked_minkowski_mul_hull(self, other: Self) -> Option<Self>;

    /// Returns the interval hull containing every point in `self / other`.
    fn checked_minkowski_div_hull(self, other: Self) -> Option<Self>;

    /// Returns the interval hull containing every point in `self * scalar`.
    fn checked_minkowski_mul_scalar_hull(self, scalar: Self::CoordType) -> Option<Self>;

    /// Returns the interval hull containing every point in `self / scalar`.
    fn checked_minkowski_div_scalar_hull(self, scalar: Self::CoordType) -> Option<Self>;
}

/// Saturating Minkowski operations whose results remain closed-open integer
/// intervals after endpoint arithmetic is clamped to the representable domain.
///
/// These methods apply saturating arithmetic to the interval bounds rather
/// than returning an error on overflow or underflow.
///
/// When saturation clips a bound, the returned interval is the representable
/// saturated result, not necessarily the exact unconstrained mathematical
/// image.
///
/// Returns `None` when saturation collapses the resulting interval into an
/// empty or otherwise invalid closed-open interval.
pub trait COSaturatingMinkowskiLinear: COPrimitive + Sized {
    /// Returns the saturated Minkowski sum `self + other`.
    ///
    /// Both result bounds are computed with saturating addition.
    fn saturating_minkowski_add(self, other: Self) -> Option<Self>;

    /// Returns the saturated Minkowski subtraction `self - other`.
    ///
    /// Both result bounds are computed with saturating subtraction.
    fn saturating_minkowski_sub(self, other: Self) -> Option<Self>;

    /// Returns the saturated translation `self + scalar`.
    ///
    /// Both interval bounds are shifted with saturating addition.
    fn saturating_minkowski_add_scalar(self, scalar: Self::CoordType) -> Option<Self>;

    /// Returns the saturated translation `self - scalar`.
    ///
    /// Both interval bounds are shifted with saturating subtraction.
    fn saturating_minkowski_sub_scalar(self, scalar: Self::CoordType) -> Option<Self>;
}

/// Saturating interval hulls of non-linear Minkowski images.
///
/// For discrete integer intervals, multiplication and division may produce
/// non-contiguous point sets. These methods first compute a containing
/// interval hull and apply saturating endpoint arithmetic where needed.
///
/// When saturation clips a bound, the returned interval is a representable
/// saturated hull rather than the exact unconstrained mathematical image.
///
/// Returns `None` when the operation is undefined, such as division by zero,
/// or when saturation collapses the resulting hull into an empty or otherwise
/// invalid closed-open interval.
pub trait COSaturatingMinkowskiHull: COPrimitive + Sized {
    /// Returns the saturated interval hull of `self * other`.
    ///
    /// Endpoint products are computed with saturating multiplication.
    fn saturating_minkowski_mul_hull(self, other: Self) -> Option<Self>;

    /// Returns the saturated interval hull of `self / other`.
    ///
    /// Returns `None` when the divisor interval contains zero in a position
    /// that makes the interval division undefined.
    fn saturating_minkowski_div_hull(self, other: Self) -> Option<Self>;

    /// Returns the saturated interval hull of `self * scalar`.
    ///
    /// Endpoint products are computed with saturating multiplication.
    fn saturating_minkowski_mul_scalar_hull(self, scalar: Self::CoordType) -> Option<Self>;

    /// Returns the saturated interval hull of `self / scalar`.
    ///
    /// Returns `None` when `scalar` is zero.
    fn saturating_minkowski_div_scalar_hull(self, scalar: Self::CoordType) -> Option<Self>;
}

/// Complete closed-open integer interval capability required by interval sets.
pub trait IntCO: COConstruct + COBounds + COPredicates + COAlgebra + COMeasure {}

impl<T> IntCO for T where T: COConstruct + COBounds + COPredicates + COAlgebra + COMeasure {}
