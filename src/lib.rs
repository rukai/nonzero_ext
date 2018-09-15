//! # Traits to represent generic nonzero integer types
//!
//! Rust ships with non-zero integer types now, which let programmers
//! promise (memory-savingly!) that a number can never be zero. That's
//! great, but sadly the standard library has no traits you can use to
//! represent all the non-zero integer types.
//!
//! # Examples
//!
//! Where this lack of traits in the standard library becomes
//! problematic is if you want to write a function that takes a vector
//! of integers, and that returns a vector of the corresponding
//! non-zero integer types, minus any elements that were zero in the
//! original. You can write that with the standard library quite
//! easily for concrete types:
//!
//! ``` rust
//! # use core::num::NonZeroU8;
//! fn only_nonzeros(v: Vec<u8>) -> Vec<NonZeroU8> {
//!   let out: Vec<NonZeroU8> = v
//!        .into_iter()
//!        .filter_map(|n| NonZeroU8::new(n))
//!        .collect();
//!   out
//! }
//! let expected: Vec<NonZeroU8> = vec![NonZeroU8::new(20).unwrap(), NonZeroU8::new(5).unwrap()];
//! assert_eq!(expected, only_nonzeros(vec![0, 20, 5]));
//! ```
//!
//! But what if you want to allow this function to work with any
//! integer type that has a corresponding non-zero type? This crate
//! can help:
//!
//! ``` rust
//! # use core::num::{NonZeroU8, NonZeroU32};
//! # use nonzero_ext::{NonZeroAble};
//! fn only_nonzeros<I>(v: Vec<I>) -> Vec<I::NonZero>
//! where
//!   I: Sized + NonZeroAble
//! {
//!   let out: Vec<I::NonZero> = v
//!        .into_iter()
//!        .filter_map(|n| n.as_nonzero())
//!        .collect();
//!   out
//! }
//!
//! // It works for `u8`:
//! let input_u8: Vec<u8> = vec![0, 20, 5];
//! let expected_u8: Vec<NonZeroU8> = vec![NonZeroU8::new(20).unwrap(), NonZeroU8::new(5).unwrap()];
//! assert_eq!(expected_u8, only_nonzeros(input_u8));
//!
//! // And it works for `u32`:
//! let input_u32: Vec<u32> = vec![0, 20, 5];
//! let expected_u32: Vec<NonZeroU32> = vec![NonZeroU32::new(20).unwrap(), NonZeroU32::new(5).unwrap()];
//! assert_eq!(expected_u32, only_nonzeros(input_u32));
//! ```
//!

use core::num::NonZeroUsize;
use core::num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8};

macro_rules! impl_nonzeroness {
    ($trait_name:ident, $nonzero_type:ty, $wrapped:ty) => {
        impl $trait_name for $nonzero_type {
            type Primitive = $wrapped;

            #[inline]
            fn new(n: $wrapped) -> Option<Self> {
                Self::new(n)
            }

            #[inline]
            fn get(self) -> Self::Primitive {
                <$nonzero_type>::get(self)
            }
        }
    };
}

/// A trait identifying a non-zero integral type. It is useful mostly
/// in order to give to genericized helper functions as `impl NonZero`
/// arguments.
pub trait NonZero {
    /// The primitive type (e.g. `u8`) underlying this integral type.
    type Primitive;

    /// Creates a new non-zero object from an integer that might be
    /// zero.
    fn new(n: Self::Primitive) -> Option<Self>
    where
        Self: Sized;

    /// Returns the value as a primitive type.
    fn get(self) -> Self::Primitive;
}

impl_nonzeroness!(NonZero, NonZeroU8, u8);
impl_nonzeroness!(NonZero, NonZeroU16, u16);
impl_nonzeroness!(NonZero, NonZeroU32, u32);
impl_nonzeroness!(NonZero, NonZeroU64, u64);
impl_nonzeroness!(NonZero, NonZeroU128, u128);
impl_nonzeroness!(NonZero, NonZeroUsize, usize);

/// A trait identifying integral types that have a non-zeroable
/// equivalent.
pub trait NonZeroAble {
    /// The concrete non-zero type represented by an implementation of
    /// this trait. For example, for `u8`'s implementation, it is
    /// `NonZeroU8`.
    type NonZero: crate::NonZero;

    /// Converts the integer to its non-zero equivalent.
    ///
    /// # Examples
    ///
    /// ### Trying to convert zero
    /// ``` rust
    /// # use nonzero_ext::NonZeroAble;
    /// let n: u16 = 0;
    /// assert_eq!(n.as_nonzero(), None);
    /// ```
    ///
    /// ### Converting a non-zero value
    /// ``` rust
    /// # use nonzero_ext::NonZeroAble;
    /// # use core::num::NonZeroUsize;
    /// let n: usize = 20;
    /// let non0n: NonZeroUsize = n.as_nonzero().expect("should result in a converted value");
    /// assert_eq!(non0n.get(), 20);
    /// ```
    fn as_nonzero(self) -> Option<Self::NonZero>;
}

macro_rules! impl_nonzeroable {
    ($trait_name:ident, $nonzero_type: ty, $nonzeroable_type:ty) => {
        impl $trait_name for $nonzeroable_type {
            type NonZero = $nonzero_type;

            fn as_nonzero(self) -> Option<$nonzero_type> {
                Self::NonZero::new(self)
            }
        }
    };
}

impl_nonzeroable!(NonZeroAble, NonZeroU8, u8);
impl_nonzeroable!(NonZeroAble, NonZeroU16, u16);
impl_nonzeroable!(NonZeroAble, NonZeroU32, u32);
impl_nonzeroable!(NonZeroAble, NonZeroU64, u64);
impl_nonzeroable!(NonZeroAble, NonZeroU128, u128);
impl_nonzeroable!(NonZeroAble, NonZeroUsize, usize);