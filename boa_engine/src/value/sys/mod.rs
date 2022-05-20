//! This module contains various implementations of the [`JsValue`] type, and
//! type redefinitions to select the requested [`JsValue`] implementation at
//! compile time, using features.

use crate::{object::JsObject, JsBigInt, JsString, JsSymbol};
use std::mem::ManuallyDrop;

// Minimum required definition for a correct `JsValue` implementation:

// pub const fn undefined() -> Self;
// pub const fn null() -> Self;
// pub const fn nan() -> Self;
// pub fn rational(f64) -> Self;
// pub fn integer(i32) -> Self;
// pub fn boolean(bool) -> Self;
// pub fn object(JsObject) -> Self;
// pub fn string(JsString) -> Self;
// pub fn symbol(JsSymbol) -> Self;
// pub fn bigint(JsBigInt) -> Self;

// pub fn as_rational(&self) -> Option<f64>;
// pub fn as_i32(&self) -> Option<i32>;
// pub fn as_boolean(&self) -> Option<bool>;
// pub fn as_object(&self) -> Option<Ref<'_, JsObject>>;
// pub fn as_string(&self) -> Option<Ref<'_, JsString>>;
// pub fn as_symbol(&self) -> Option<Ref<'_, JsSymbol>>;
// pub fn as_bigint(&self) -> Option<Ref<'_, JsBigInt>>;

// pub fn is_undefined(&self) -> bool;
// pub fn is_null(&self) -> bool;
// pub fn is_nan(&self) -> bool;
// pub fn is_rational(&self) -> bool;
// pub fn is_i32(&self) -> bool;
// pub fn is_boolean(&self) -> bool;
// pub fn is_object(&self) -> bool;
// pub fn is_string(&self) -> bool;
// pub fn is_symbol(&self) -> bool;
// pub fn is_bigint(&self) -> bool;

// pub fn variant(&self) -> JsVariant<'_>;

// Ref<'a, T> type

// TODO: Represent this abstraction as a trait. Requires https://github.com/rust-lang/rust/pull/96709

#[cfg(doc)]
pub mod default;

#[cfg(doc)]
pub mod nan_boxed;

cfg_if::cfg_if! {
    if #[cfg(all(target_arch = "x86_64", target_pointer_width = "64", feature = "nan_boxing"))] {
        #[path = "nan_boxed.rs"]
        mod r#impl;
    } else {
        #[path = "default.rs"]
        mod r#impl;
    }
}

#[cfg(doc)]
pub use default::*;

#[cfg(not(doc))]
pub type JsValue = r#impl::JsValue;

#[cfg(not(doc))]
pub type Ref<'a, T> = r#impl::Ref<'a, T>;

/// Return value of the [`JsValue::variant`] method.
///
/// Represents either a primitive value ([`bool`], [`f64`], [`i32`]) or a reference
/// to a heap allocated value ([`JsString`], [`JsSymbol`]).
///
/// References to heap allocated values are represented by [`Ref`], since
/// more exotic implementations of [`JsValue`] such as nan-boxed ones cannot
/// effectively return references.
#[derive(Debug)]
pub enum JsVariant<'a> {
    Null,
    Undefined,
    Rational(f64),
    Integer(i32),
    Boolean(bool),
    String(Ref<'a, JsString>),
    Symbol(Ref<'a, JsSymbol>),
    BigInt(Ref<'a, JsBigInt>),
    Object(Ref<'a, JsObject>),
}

/// This abstracts over every pointer type boxed inside `NaN` values.
///
/// # Safety
///
/// Non-exhaustive list of situations that could cause undefined behaviour:
/// - Returning an invalid `*mut ()`.
/// - Returning a `ManuallyDrop<Self>` that doesn't correspond with the provided
/// `ptr`.
/// - Dropping `ty` before returning its pointer.
pub(crate) unsafe trait PointerType {
    unsafe fn from_void_ptr(ptr: *mut ()) -> ManuallyDrop<Self>;

    unsafe fn into_void_ptr(ty: ManuallyDrop<Self>) -> *mut ();
}
