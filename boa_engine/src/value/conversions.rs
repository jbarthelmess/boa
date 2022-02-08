use crate::JsString;

use super::{Display, JsValue};

impl From<&Self> for JsValue {
    #[inline]
    fn from(value: &Self) -> Self {
        value.clone()
    }
}

impl From<u8> for JsValue {
    #[inline]
    fn from(value: u8) -> Self {
        Self::from(value as i32)
    }
}

impl From<i8> for JsValue {
    #[inline]
    fn from(value: i8) -> Self {
        Self::from(value as i32)
    }
}

impl From<u16> for JsValue {
    #[inline]
    fn from(value: u16) -> Self {
        Self::from(value as i32)
    }
}

impl From<i16> for JsValue {
    #[inline]
    fn from(value: i16) -> Self {
        Self::from(value as i32)
    }
}

impl From<u32> for JsValue {
    #[inline]
    fn from(value: u32) -> Self {
        if let Ok(integer) = i32::try_from(value) {
            Self::from(integer)
        } else {
            Self::from(value as f64)
        }
    }
}

impl From<usize> for JsValue {
    #[inline]
    fn from(value: usize) -> Self {
        if let Ok(value) = i32::try_from(value) {
            Self::from(value)
        } else {
            Self::from(value as f64)
        }
    }
}

impl From<isize> for JsValue {
    #[inline]
    fn from(value: isize) -> Self {
        if let Ok(value) = i32::try_from(value) {
            Self::from(value)
        } else {
            Self::from(value as f64)
        }
    }
}

impl From<u64> for JsValue {
    #[inline]
    fn from(value: u64) -> Self {
        if let Ok(value) = i32::try_from(value) {
            Self::from(value)
        } else {
            Self::from(value as f64)
        }
    }
}

impl From<i64> for JsValue {
    #[inline]
    fn from(value: i64) -> Self {
        if let Ok(value) = i32::try_from(value) {
            Self::from(value)
        } else {
            Self::from(value as f64)
        }
    }
}

impl From<f32> for JsValue {
    #[allow(clippy::float_cmp)]
    #[inline]
    fn from(value: f32) -> Self {
        // if value as i32 as f64 == value {
        //     Self::Integer(value as i32)
        // } else {
        Self::from(value as f64)
        // }
    }
}

impl From<char> for JsValue {
    #[inline]
    fn from(value: char) -> Self {
        Self::from(value.to_string())
    }
}

impl From<&str> for JsValue {
    #[inline]
    fn from(string: &str) -> Self {
        Self::from(JsString::new(string))
    }
}

impl From<Box<str>> for JsValue {
    #[inline]
    fn from(string: Box<str>) -> Self {
        Self::from(JsString::new(string))
    }
}

impl From<&String> for JsValue {
    #[inline]
    fn from(string: &String) -> Self {
        Self::from(JsString::new(string.as_str()))
    }
}

impl From<String> for JsValue {
    #[inline]
    fn from(string: String) -> Self {
        Self::from(JsString::new(string))
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct TryFromObjectError;

impl Display for TryFromObjectError {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Could not convert value to an Object type")
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct TryFromCharError;

impl Display for TryFromCharError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Could not convert value to a char type")
    }
}

pub(crate) trait IntoOrUndefined {
    fn into_or_undefined(self) -> JsValue;
}

impl<T> IntoOrUndefined for Option<T>
where
    T: Into<JsValue>,
{
    fn into_or_undefined(self) -> JsValue {
        self.map_or_else(JsValue::undefined, Into::into)
    }
}
