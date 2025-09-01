//! Non-zero sizes.
//!
//! This crate provides the [`Size`] type representing non-zero sizes, along with
//! [`size!`] and [`const_size!`] macros used for constructing values of said type.

#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![no_std]

use core::num::NonZero;

/// The error message used for when the provided size is zero.
pub const NON_ZERO: &str = "expected non-zero size";

/// Represents non-zero sizes, [`NonZero<usize>`].
pub type Size = NonZero<usize>;

/// Constructs [`Size`], panicking if the provided size is zero.
///
/// # Examples
///
/// ```
/// use non_zero_size::size;
///
/// let nekit = 13;
///
/// let non_zero = size!(nekit);
///
/// assert_eq!(non_zero.get(), nekit);
/// ```
///
/// Panicking on zero:
///
/// ```should_panic
/// use non_zero_size::size;
///
/// let never = size!(0);
/// ```
///
/// Failing compilation on zero in `const` contexts (see also [`const_size`]):
///
/// ```compile_fail
/// use non_zero_size::size;
///
/// let never = const { size!(0) };
/// ```
#[macro_export]
macro_rules! size {
    ($value: expr) => {
        $crate::Size::new($value).expect($crate::NON_ZERO)
    };
}

/// Constantly constructs [`Size`], failing compilation if the provided size is zero.
///
/// Note that the provided expression must be const-evaluatable, else the compilation will fail.
///
/// # Examples
///
/// ```
/// use non_zero_size::const_size;
///
/// const NICE: usize = 69;
///
/// let non_zero = const_size!(NICE);
///
/// assert_eq!(non_zero.get(), NICE);
/// ```
///
/// Failing compilation on zero:
///
/// ```compile_fail
/// use non_zero_size::const_size;
///
/// let never = const_size!(0);
/// ```
#[macro_export]
macro_rules! const_size {
    ($value: expr) => {
        const { $crate::size!($value) }
    };
}
