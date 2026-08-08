#![no_std]
#![forbid(unsafe_code)]

use core::ops::Add;
use core::ops::AddAssign;
use core::ops::Div;
use core::ops::DivAssign;
use core::ops::Mul;
use core::ops::MulAssign;
use core::ops::Rem;
use core::ops::RemAssign;
use core::ops::Sub;
use core::ops::SubAssign;

macro_rules! impl_op_trait_for_ref_combinations {
    (inner_t: $inner_t:ident, type: $T:ident, trait: $trait:ident, trait_fn: $trait_fn:ident, inner_fn: $inner_fn: ident) => {
        impl $trait for $T {
            type Output = $T;

            #[inline(always)]
            fn $trait_fn(self, rhs: $T) -> Self::Output {
                $T {
                    inner: $inner_t::$inner_fn(self.inner, rhs.inner),
                }
            }
        }

        impl $trait<&$T> for $T {
            type Output = $T;

            #[inline(always)]
            fn $trait_fn(self, rhs: &$T) -> Self::Output {
                $T {
                    inner: $inner_t::$inner_fn(self.inner, rhs.inner),
                }
            }
        }

        impl $trait<$T> for &$T {
            type Output = $T;

            #[inline(always)]
            fn $trait_fn(self, rhs: $T) -> Self::Output {
                $T {
                    inner: $inner_t::$inner_fn(self.inner, rhs.inner),
                }
            }
        }

        impl<'a> $trait<&'a $T> for &'a $T {
            type Output = $T;

            #[inline(always)]
            fn $trait_fn(self, rhs: &$T) -> Self::Output {
                $T {
                    inner: $inner_t::$inner_fn(self.inner, rhs.inner),
                }
            }
        }
    };
}

macro_rules! impl_op_assign_trait_for_ref_combinations {
    (inner_t: $inner_t:ident, type: $T:ident, trait: $trait:ident, trait_fn: $trait_fn:ident, inner_fn: $inner_fn: ident) => {
        impl $trait for $T {
            #[inline(always)]
            fn $trait_fn(&mut self, rhs: $T) {
                *self = $T::$inner_fn(*self, rhs);
            }
        }

        impl $trait<&$T> for $T {
            #[inline(always)]
            fn $trait_fn(&mut self, rhs: &$T) {
                *self = $T::$inner_fn(*self, rhs);
            }
        }
    };
}

macro_rules! impl_algebraic_float {
    ($inner_t:ident, $af_t:ident, $alias: ident) => {
        #[expect(non_camel_case_types)]
        #[derive(Clone, Copy, Debug, PartialEq)]
        #[repr(transparent)]
        pub struct $af_t {
            inner: $inner_t,
        }

        pub type $alias = $af_t;

        impl $af_t {
            pub const fn new(inner: $inner_t) -> Self {
                Self { inner }
            }
        }

        impl_op_trait_for_ref_combinations! { inner_t: $inner_t, type: $af_t, trait: Add, trait_fn: add, inner_fn: algebraic_add }
        impl_op_trait_for_ref_combinations! { inner_t: $inner_t, type: $af_t, trait: Sub, trait_fn: sub, inner_fn: algebraic_sub }
        impl_op_trait_for_ref_combinations! { inner_t: $inner_t, type: $af_t, trait: Mul, trait_fn: mul, inner_fn: algebraic_mul }
        impl_op_trait_for_ref_combinations! { inner_t: $inner_t, type: $af_t, trait: Div, trait_fn: div, inner_fn: algebraic_div }
        impl_op_trait_for_ref_combinations! { inner_t: $inner_t, type: $af_t, trait: Rem, trait_fn: rem, inner_fn: algebraic_rem }

        impl_op_assign_trait_for_ref_combinations! { inner_t: $inner_t, type: $af_t, trait: AddAssign, trait_fn: add_assign, inner_fn: add }
        impl_op_assign_trait_for_ref_combinations! { inner_t: $inner_t, type: $af_t, trait: SubAssign, trait_fn: sub_assign, inner_fn: sub }
        impl_op_assign_trait_for_ref_combinations! { inner_t: $inner_t, type: $af_t, trait: MulAssign, trait_fn: mul_assign, inner_fn: mul }
        impl_op_assign_trait_for_ref_combinations! { inner_t: $inner_t, type: $af_t, trait: DivAssign, trait_fn: div_assign, inner_fn: div }
        impl_op_assign_trait_for_ref_combinations! { inner_t: $inner_t, type: $af_t, trait: RemAssign, trait_fn: rem_assign, inner_fn: rem }
    };
}

impl_algebraic_float! {f32, af32, AlgebraicFloat32}
impl_algebraic_float! {f64, af64, AlgebraicFloat64}
