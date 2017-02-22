use abs::Abs;
use core::{f32, f64};


pub trait Approx {
    fn approx_eq(self, other: Self) -> bool;
    fn approx_ne(self, other: Self) -> bool;
}

macro_rules! trait_approx {
    ($t:ident) => (
        impl Approx for $t {
            #[inline(always)]
            fn approx_eq(self, other: Self) -> bool {self == other}
            fn approx_ne(self, other: Self) -> bool {self != other}
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

impl Approx for f32 {
    #[inline(always)]
    fn approx_eq(self, other: Self) -> bool {
        (self - other).abs() < f32::EPSILON
    }
    fn approx_ne(self, other: Self) -> bool {
        !self.approx_eq(other)
    }
}
impl Approx for f64 {
    #[inline(always)]
    fn approx_eq(self, other: Self) -> bool {
        (self - other).abs() < f64::EPSILON
    }
    fn approx_ne(self, other: Self) -> bool {
        !self.approx_eq(other)
    }
}

#[test]
fn approx() {
    use core::f32::consts::PI;

    assert_eq!((1u32).approx_eq(1u32), true);
    assert_eq!((1f32).approx_eq(PI / PI), true);
}
