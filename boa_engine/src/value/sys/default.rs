use boa_gc::{Finalize, Trace};

use super::JsVariant;

use crate::{object::JsObject, JsBigInt, JsString, JsSymbol};

/// A Javascript value
#[derive(Trace, Finalize, Debug, Clone)]
pub enum JsValue {
    /// `null` - A null value, for when a value doesn't exist.
    Null,
    /// `undefined` - An undefined value, for when a field or index doesn't exist.
    Undefined,
    /// `boolean` - A `true` / `false` value, for if a certain criteria is met.
    Boolean(bool),
    /// `String` - A UTF-8 string, such as `"Hello, world"`.
    String(JsString),
    /// `Number` - A 64-bit floating point number, such as `3.1415`
    Rational(f64),
    /// `Number` - A 32-bit integer, such as `42`.
    Integer(i32),
    /// `BigInt` - holds any arbitrary large signed integer.
    BigInt(JsBigInt),
    /// `Object` - An object, such as `Math`, represented by a binary tree of string keys to Javascript values.
    Object(JsObject),
    /// `Symbol` - A Symbol Primitive type.
    Symbol(JsSymbol),
}

impl JsValue {
    /// Creates a new `undefined` value.
    #[inline]
    pub const fn undefined() -> Self {
        Self::Undefined
    }

    /// Creates a new `null` value.
    #[inline]
    pub const fn null() -> Self {
        Self::Null
    }

    /// Creates a new number with `NaN` value.
    #[inline]
    pub const fn nan() -> Self {
        Self::Rational(f64::NAN)
    }

    #[inline]
    pub fn rational(rational: f64) -> Self {
        Self::Rational(rational)
    }

    #[inline]
    pub fn integer(integer: i32) -> Self {
        Self::Integer(integer)
    }

    #[inline]
    pub fn boolean(boolean: bool) -> Self {
        Self::Boolean(boolean)
    }

    #[inline]
    pub fn object(object: JsObject) -> Self {
        Self::Object(object)
    }

    #[inline]
    pub fn string(string: JsString) -> Self {
        Self::String(string)
    }

    #[inline]
    pub fn symbol(symbol: JsSymbol) -> Self {
        Self::Symbol(symbol)
    }

    #[inline]
    pub fn bigint(bigint: JsBigInt) -> Self {
        Self::BigInt(bigint)
    }

    #[inline]
    pub fn as_rational(&self) -> Option<f64> {
        match *self {
            Self::Rational(rational) => Some(rational),
            _ => None,
        }
    }

    #[inline]
    pub fn as_i32(&self) -> Option<i32> {
        match *self {
            Self::Integer(integer) => Some(integer),
            _ => None,
        }
    }

    #[inline]
    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            Self::Boolean(boolean) => Some(*boolean),
            _ => None,
        }
    }

    #[inline]
    pub fn as_object(&self) -> Option<Ref<'_, JsObject>> {
        match self {
            Self::Object(inner) => Some(Ref { inner }),
            _ => None,
        }
    }

    /// Returns the string if the values is a string, otherwise `None`.
    #[inline]
    pub fn as_string(&self) -> Option<Ref<'_, JsString>> {
        match self {
            Self::String(inner) => Some(Ref { inner }),
            _ => None,
        }
    }

    pub fn as_symbol(&self) -> Option<Ref<'_, JsSymbol>> {
        match self {
            Self::Symbol(inner) => Some(Ref { inner }),
            _ => None,
        }
    }

    /// Returns an optional reference to a `BigInt` if the value is a `BigInt` primitive.
    #[inline]
    pub fn as_bigint(&self) -> Option<Ref<'_, JsBigInt>> {
        match self {
            Self::BigInt(inner) => Some(Ref { inner }),
            _ => None,
        }
    }

    /// Returns true if the value is undefined.
    #[inline]
    pub fn is_undefined(&self) -> bool {
        matches!(self, Self::Undefined)
    }

    /// Returns true if the value is null.
    #[inline]
    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    /// Returns true if the value is a 64-bit floating-point number.
    #[inline]
    pub fn is_rational(&self) -> bool {
        matches!(self, Self::Rational(_))
    }

    /// Returns true if the value is any of the representations of a 64-bit floating-point `NaN`.
    #[inline]
    pub fn is_nan(&self) -> bool {
        matches!(self, Self::Rational(r) if r.is_nan())
    }

    /// Returns true if the value is a 32-bit signed integer number.
    #[inline]
    pub fn is_i32(&self) -> bool {
        matches!(self, Self::Integer(_))
    }

    /// Returns true if the value is a boolean.
    #[inline]
    pub fn is_boolean(&self) -> bool {
        matches!(self, Self::Boolean(_))
    }

    /// Returns true if the value is an object
    #[inline]
    pub fn is_object(&self) -> bool {
        matches!(self, Self::Object(_))
    }

    /// Returns true if the value is a string.
    #[inline]
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    /// Returns true if the value is a symbol.
    #[inline]
    pub fn is_symbol(&self) -> bool {
        matches!(self, Self::Symbol(_))
    }

    /// Returns true if the value is a bigint.
    #[inline]
    pub fn is_bigint(&self) -> bool {
        matches!(self, Self::BigInt(_))
    }

    pub fn variant(&self) -> JsVariant<'_> {
        match self {
            Self::Null => JsVariant::Null,
            Self::Undefined => JsVariant::Undefined,
            Self::Integer(i) => JsVariant::Integer(*i),
            Self::Rational(d) => JsVariant::Rational(*d),
            Self::Boolean(b) => JsVariant::Boolean(*b),
            Self::Object(inner) => JsVariant::Object(Ref { inner }),
            Self::String(inner) => JsVariant::String(Ref { inner }),
            Self::Symbol(inner) => JsVariant::Symbol(Ref { inner }),
            Self::BigInt(inner) => JsVariant::BigInt(Ref { inner }),
        }
    }
}

/// Represents a reference to a pointer type inside a [`JsValue`]
///
/// [`Ref`] implements [`Deref`][`std::ops::Deref`], which facilitates conversion
/// to a proper [`reference`] by using the `ref` keyword or the
/// [`Option::as_deref`][`std::option::Option::as_deref`] method.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ref<'a, T> {
    inner: &'a T,
}

// Lift `Ref` over `AsRef`, since implementing `AsRef<T>` would override the
// `as_ref` implementations of `T`.
impl<U, T> AsRef<U> for Ref<'_, T>
where
    T: AsRef<U>,
{
    #[inline]
    fn as_ref(&self) -> &U {
        <T as AsRef<U>>::as_ref(self.inner)
    }
}

impl<T> std::ops::Deref for Ref<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

impl<T: PartialEq> PartialEq<T> for Ref<'_, T> {
    #[inline]
    fn eq(&self, other: &T) -> bool {
        self.inner == other
    }
}

impl<T> std::borrow::Borrow<T> for Ref<'_, T> {
    #[inline]
    fn borrow(&self) -> &T {
        self.inner
    }
}
