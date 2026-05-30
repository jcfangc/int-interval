macro_rules! __impl_co_basic_forwarding {
    ($interval:ty, $coord:ty, $measure:ty) => {
        impl $crate::traits::COPrimitive for $interval {
            type CoordType = $coord;
            type MeasureType = $measure;
        }

        impl $crate::traits::COConstruct for $interval {
            #[inline]
            fn try_new(start: $coord, end_excl: $coord) -> Option<Self> {
                <$interval>::try_new(start, end_excl)
            }

            #[inline]
            unsafe fn new_unchecked(start: $coord, end_excl: $coord) -> Self {
                unsafe { <$interval>::new_unchecked(start, end_excl) }
            }
        }

        impl $crate::traits::COMidpointConstruct for $interval {
            #[inline]
            fn checked_from_midpoint_len(mid: $coord, len: $measure) -> Option<Self> {
                <$interval>::checked_from_midpoint_len(mid, len)
            }

            #[inline]
            fn saturating_from_midpoint_len(mid: $coord, len: $measure) -> Option<Self> {
                <$interval>::saturating_from_midpoint_len(mid, len)
            }
        }

        impl $crate::traits::COStartLenConstruct for $interval {
            #[inline]
            fn checked_from_start_len(start: $coord, len: $measure) -> Option<Self> {
                <$interval>::checked_from_start_len(start, len)
            }

            #[inline]
            fn saturating_from_start_len(start: $coord, len: $measure) -> Option<Self> {
                <$interval>::saturating_from_start_len(start, len)
            }
        }

        impl $crate::traits::COBounds for $interval {
            #[inline]
            fn start(self) -> $coord {
                <$interval>::start(self)
            }

            #[inline]
            fn end_excl(self) -> $coord {
                <$interval>::end_excl(self)
            }

            #[inline]
            fn end_incl(self) -> $coord {
                <$interval>::end_incl(self)
            }
        }

        impl $crate::traits::COMeasure for $interval {
            #[inline]
            fn len(self) -> $measure {
                <$interval>::len(self)
            }
        }

        impl $crate::traits::COMidpoint for $interval {
            #[inline]
            fn midpoint(self) -> $coord {
                <$interval>::midpoint(self)
            }
        }

        impl $crate::traits::COPredicates for $interval {
            #[inline]
            fn contains(self, x: $coord) -> bool {
                <$interval>::contains(self, x)
            }

            #[inline]
            fn contains_interval(self, other: Self) -> bool {
                <$interval>::contains_interval(self, other)
            }

            #[inline]
            fn intersects(self, other: Self) -> bool {
                <$interval>::intersects(self, other)
            }

            #[inline]
            fn is_adjacent(self, other: Self) -> bool {
                <$interval>::is_adjacent(self, other)
            }

            #[inline]
            fn is_contiguous_with(self, other: Self) -> bool {
                <$interval>::is_contiguous_with(self, other)
            }
        }

        impl $crate::traits::CORange for $interval {
            #[inline]
            fn to_range(self) -> core::ops::Range<$coord> {
                <$interval>::to_range(self)
            }

            #[inline]
            fn iter(self) -> core::ops::Range<$coord> {
                <$interval>::iter(self)
            }
        }
    };
}

macro_rules! __impl_co_algebra_forwarding {
    ($interval:ty) => {
        impl $crate::traits::COAlgebra for $interval {
            #[inline]
            fn intersection(self, other: Self) -> Option<Self> {
                <$interval>::intersection(self, other)
            }

            #[inline]
            fn convex_hull(self, other: Self) -> Self {
                <$interval>::convex_hull(self, other)
            }

            #[inline]
            fn between(self, other: Self) -> Option<Self> {
                <$interval>::between(self, other)
            }

            #[inline]
            fn union(self, other: Self) -> $crate::res::OneTwo<Self> {
                <$interval>::union(self, other)
            }

            #[inline]
            fn difference(self, other: Self) -> $crate::res::ZeroOneTwo<Self> {
                <$interval>::difference(self, other)
            }

            #[inline]
            fn symmetric_difference(self, other: Self) -> $crate::res::ZeroOneTwo<Self> {
                <$interval>::symmetric_difference(self, other)
            }
        }
    };
}

macro_rules! __impl_co_checked_minkowski_forwarding {
    ($interval:ty, $coord:ty) => {
        impl $crate::traits::COCheckedMinkowskiLinear for $interval {
            #[inline]
            fn checked_minkowski_add(self, other: Self) -> Option<Self> {
                <$interval>::checked_minkowski_add(self, other)
            }

            #[inline]
            fn checked_minkowski_sub(self, other: Self) -> Option<Self> {
                <$interval>::checked_minkowski_sub(self, other)
            }

            #[inline]
            fn checked_minkowski_add_scalar(self, scalar: $coord) -> Option<Self> {
                <$interval>::checked_minkowski_add_scalar(self, scalar)
            }

            #[inline]
            fn checked_minkowski_sub_scalar(self, scalar: $coord) -> Option<Self> {
                <$interval>::checked_minkowski_sub_scalar(self, scalar)
            }
        }

        impl $crate::traits::COCheckedMinkowskiHull for $interval {
            #[inline]
            fn checked_minkowski_mul_hull(self, other: Self) -> Option<Self> {
                <$interval>::checked_minkowski_mul_hull(self, other)
            }

            #[inline]
            fn checked_minkowski_div_hull(self, other: Self) -> Option<Self> {
                <$interval>::checked_minkowski_div_hull(self, other)
            }

            #[inline]
            fn checked_minkowski_mul_scalar_hull(self, scalar: $coord) -> Option<Self> {
                <$interval>::checked_minkowski_mul_scalar_hull(self, scalar)
            }

            #[inline]
            fn checked_minkowski_div_scalar_hull(self, scalar: $coord) -> Option<Self> {
                <$interval>::checked_minkowski_div_scalar_hull(self, scalar)
            }
        }
    };
}

macro_rules! __impl_co_saturating_minkowski_forwarding {
    ($interval:ty, $coord:ty) => {
        impl $crate::traits::COSaturatingMinkowskiLinear for $interval {
            #[inline]
            fn saturating_minkowski_add(self, other: Self) -> Option<Self> {
                <$interval>::saturating_minkowski_add(self, other)
            }

            #[inline]
            fn saturating_minkowski_sub(self, other: Self) -> Option<Self> {
                <$interval>::saturating_minkowski_sub(self, other)
            }

            #[inline]
            fn saturating_minkowski_add_scalar(self, scalar: $coord) -> Option<Self> {
                <$interval>::saturating_minkowski_add_scalar(self, scalar)
            }

            #[inline]
            fn saturating_minkowski_sub_scalar(self, scalar: $coord) -> Option<Self> {
                <$interval>::saturating_minkowski_sub_scalar(self, scalar)
            }
        }

        impl $crate::traits::COSaturatingMinkowskiHull for $interval {
            #[inline]
            fn saturating_minkowski_mul_hull(self, other: Self) -> Option<Self> {
                <$interval>::saturating_minkowski_mul_hull(self, other)
            }

            #[inline]
            fn saturating_minkowski_div_hull(self, other: Self) -> Option<Self> {
                <$interval>::saturating_minkowski_div_hull(self, other)
            }

            #[inline]
            fn saturating_minkowski_mul_scalar_hull(self, scalar: $coord) -> Option<Self> {
                <$interval>::saturating_minkowski_mul_scalar_hull(self, scalar)
            }

            #[inline]
            fn saturating_minkowski_div_scalar_hull(self, scalar: $coord) -> Option<Self> {
                <$interval>::saturating_minkowski_div_scalar_hull(self, scalar)
            }
        }
    };
}

macro_rules! impl_co_forwarding {
    ($interval:ty, $coord:ty, $measure:ty) => {
        $crate::traits::forwarding::__impl_co_basic_forwarding!($interval, $coord, $measure);
        $crate::traits::forwarding::__impl_co_algebra_forwarding!($interval);
        $crate::traits::forwarding::__impl_co_checked_minkowski_forwarding!($interval, $coord);
        $crate::traits::forwarding::__impl_co_saturating_minkowski_forwarding!($interval, $coord);
    };
}

pub(crate) use __impl_co_algebra_forwarding;
pub(crate) use __impl_co_basic_forwarding;
pub(crate) use __impl_co_checked_minkowski_forwarding;
pub(crate) use __impl_co_saturating_minkowski_forwarding;
pub(crate) use impl_co_forwarding;
