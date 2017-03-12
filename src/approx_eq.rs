use core::ops::Sub;
use core::{f32, f64};

use zero::Zero;


pub trait ApproxEq: Clone + PartialOrd + Zero + Sub<Self, Output=Self> {
    #[inline(always)]
    fn approx_eq(&self, other: &Self) -> bool {
        self.approx_eq_tolerance(other, &Zero::zero())
    }
    #[inline(always)]
    fn approx_ne(&self, other: &Self) -> bool {
        !self.approx_eq(other)
    }

    #[inline]
    fn approx_eq_tolerance(&self, other: &Self, tolerance: &Self) -> bool {
        let a = self.clone();
        let b = other.clone();

        if a > b {
            (a - b) < tolerance.clone()
        } else {
            (b - a) < tolerance.clone()
        }
    }
    #[inline(always)]
    fn approx_ne_tolerance(&self, other: &Self, tolerance: &Self) -> bool {
        !self.approx_eq_tolerance(other, tolerance)
    }
}

macro_rules! trait_approx {
    ($t:ident) => (
        impl ApproxEq for $t {
            #[inline(always)]
            fn approx_eq(&self, other: &Self) -> bool {
                self == other
            }
        }
    );
}

trait_approx!(usize);
trait_approx!(u8);
trait_approx!(u16);
trait_approx!(u32);
trait_approx!(u64);

trait_approx!(isize);
trait_approx!(i8);
trait_approx!(i16);
trait_approx!(i32);
trait_approx!(i64);

impl ApproxEq for f32 {
    #[inline]
    fn approx_eq(&self, other: &Self) -> bool {
        self.approx_eq_tolerance(other, &f32::EPSILON)
    }
}
impl ApproxEq for f64 {
    #[inline]
    fn approx_eq(&self, other: &Self) -> bool {
        self.approx_eq_tolerance(other, &f64::EPSILON)
    }
}

#[test]
fn approx_eq() {
    use core::f32::consts::PI;

    assert_eq!((1u32).approx_eq(&1u32), true);
    assert_eq!((1f32).approx_eq(&(PI / PI)), true);
}
