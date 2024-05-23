use std::os::raw::c_void;

use crate::{Color, ToXfconfValue, TryFromXfconfValue, XfconfGValueExt};
use glib::{prelude::*, translate::*, Type};

macro_rules! impl_try_from_xfconf_value_simple {
    ($ty:ty) => {
        impl TryFromXfconfValue for $ty {
            fn try_from_xfconf_value(value: &glib::Value) -> Option<Self> {
                value.get().ok()
            }

            fn xfconf_display_name() -> &'static str {
                stringify!($ty)
            }
        }
    };
}

macro_rules! impl_try_from_xfconf_value_int {
    ($ty:ty) => {
        impl TryFromXfconfValue for $ty {
            fn try_from_xfconf_value(value: &glib::Value) -> Option<Self> {
                match value.type_() {
                    Type::U8 | Type::U32 | Type::U64 | Type::U_LONG => value
                        .get::<u64>()
                        .ok()
                        .and_then(|v| <$ty>::try_from(v).ok()),
                    Type::I8 | Type::I32 | Type::I64 | Type::I_LONG => value
                        .get::<i64>()
                        .ok()
                        .and_then(|v| <$ty>::try_from(v).ok()),
                    Type::STRING => value.get::<&str>().ok().and_then(|v| v.parse::<$ty>().ok()),
                    Type::BOOL => value.get::<bool>().ok().map(|v| if v { 1 } else { 0 }),
                    Type::FLAGS => {
                        if std::mem::size_of::<$ty>() < std::mem::size_of::<u32>() {
                            None
                        } else {
                            value
                                .get::<u32>()
                                .ok()
                                .and_then(|v| <$ty>::try_from(v).ok())
                        }
                    }
                    x if x == crate::XfconfGType::u16() => {
                        value.get_u16().and_then(|v| <$ty>::try_from(v).ok())
                    }
                    x if x == crate::XfconfGType::i16() => {
                        value.get_i16().and_then(|v| <$ty>::try_from(v).ok())
                    }
                    _ => None,
                }
            }

            fn xfconf_display_name() -> &'static str {
                stringify!($ty)
            }
        }
    };
}

macro_rules! impl_to_xfconf_value_simple {
    ($ty:ty) => {
        impl ToXfconfValue for $ty {
            fn to_xfconf_value(&self) -> glib::Value {
                self.to_value()
            }
        }
    };
}

macro_rules! impl_to_xfconf_value_size {
    ($ty:ty, $t64:ty, $t32:ty, $e16:ident, $t8:ty) => {
        impl ToXfconfValue for $ty {
            fn to_xfconf_value(&self) -> glib::Value {
                #[cfg(target_pointer_width = "8")]
                let value = (*self as $t8).to_value();
                #[cfg(target_pointer_width = "16")]
                let value = crate::XfconfGType::$e16(*self);
                #[cfg(target_pointer_width = "32")]
                let value = (*self as $t32).to_value();
                #[cfg(target_pointer_width = "64")]
                let value = (*self as $t64).to_value();

                value
            }
        }
    };
}

macro_rules! impl_to_xfconf_value_short {
    ($ty:ty, $construct:path) => {
        impl ToXfconfValue for $ty {
            fn to_xfconf_value(&self) -> glib::Value {
                $construct(*self)
            }
        }
    };
}

impl_try_from_xfconf_value_int!(u8);
impl_try_from_xfconf_value_int!(u16);
impl_try_from_xfconf_value_int!(u32);
impl_try_from_xfconf_value_int!(u64);
impl_try_from_xfconf_value_int!(i8);
impl_try_from_xfconf_value_int!(i16);
impl_try_from_xfconf_value_int!(i32);
impl_try_from_xfconf_value_int!(i64);
impl_try_from_xfconf_value_simple!(String);
impl_try_from_xfconf_value_simple!(glib::GString);

impl TryFromXfconfValue for bool {
    fn try_from_xfconf_value(value: &glib::Value) -> Option<Self> {
        match value.type_() {
            Type::U8 | Type::U32 | Type::U64 | Type::U_LONG => {
                value.get::<u64>().ok().map(|v| v != 0)
            }
            Type::I8 | Type::I32 | Type::I64 | Type::I_LONG => {
                value.get::<i64>().ok().map(|v| v != 0)
            }
            Type::STRING => value
                .get::<&str>()
                .ok()
                .and_then(|v| v.parse::<bool>().ok()),
            Type::BOOL => value.get::<bool>().ok(),
            Type::F32 => value.get::<f32>().ok().map(|v| v != 0f32),
            Type::F64 => value.get::<f64>().ok().map(|v| v != 0f64),
            Type::UNIT => Some(false),
            _ => None,
        }
    }

    fn xfconf_display_name() -> &'static str {
        "bool"
    }
}

impl TryFromXfconfValue for f32 {
    fn try_from_xfconf_value(value: &glib::Value) -> Option<Self> {
        match value.type_() {
            Type::BOOL => value.get::<bool>().ok().map(f32::from),
            Type::F32 => value.get::<f32>().ok(),
            Type::U8 => value.get::<u8>().ok().map(f32::from),
            Type::I8 => value.get::<i8>().ok().map(f32::from),
            Type::STRING => value.get::<&str>().ok().and_then(|v| v.parse::<f32>().ok()),
            x if x == crate::XfconfGType::u16() => value.get_u16().map(f32::from),
            x if x == crate::XfconfGType::i16() => value.get_i16().map(f32::from),
            _ => None,
        }
    }

    fn xfconf_display_name() -> &'static str {
        "f32"
    }
}

impl TryFromXfconfValue for f64 {
    fn try_from_xfconf_value(value: &glib::Value) -> Option<Self> {
        match value.type_() {
            Type::BOOL => value.get::<bool>().ok().map(f64::from),
            Type::F32 => value.get::<f32>().ok().map(f64::from),
            Type::F64 => value.get::<f64>().ok(),
            Type::U8 => value.get::<u8>().ok().map(f64::from),
            Type::U32 => value.get::<u32>().ok().map(f64::from),
            Type::I8 => value.get::<i8>().ok().map(f64::from),
            Type::I32 => value.get::<i32>().ok().map(f64::from),
            Type::STRING => value.get::<&str>().ok().and_then(|v| v.parse::<f64>().ok()),
            x if x == crate::XfconfGType::u16() => value.get_u16().map(f64::from),
            x if x == crate::XfconfGType::i16() => value.get_i16().map(f64::from),
            _ => None,
        }
    }

    fn xfconf_display_name() -> &'static str {
        "f64"
    }
}

impl TryFromXfconfValue for Color {
    fn try_from_xfconf_value(value: &glib::Value) -> Option<Self> {
        Color::from_value(value)
    }

    fn xfconf_display_name() -> &'static str {
        "color"
    }
}

impl TryFromXfconfValue for glib::Value {
    fn try_from_xfconf_value(value: &glib::Value) -> Option<Self> {
        Some(value.clone())
    }

    fn xfconf_display_name() -> &'static str {
        "value"
    }
}

impl<T: TryFromXfconfValue> TryFromXfconfValue for Vec<T> {
    fn try_from_xfconf_value(value: &glib::Value) -> Option<Self> {
        if value.type_() == ptr_array_gtype() {
            let array_ptr = unsafe { glib::gobject_ffi::g_value_get_boxed(value.as_ptr()) }
                as *mut glib::ffi::GPtrArray;
            if !array_ptr.is_null() {
                let values: Vec<glib::Value> =
                    unsafe { FromGlibPtrArrayContainerAsVec::from_glib_none_as_vec(array_ptr) };
                let len = values.len();
                let values = values
                    .into_iter()
                    .flat_map(|v| {
                        T::try_from_xfconf_value(&v)
                            .map(|v| vec![v])
                            .unwrap_or(vec![])
                    })
                    .collect::<Vec<T>>();
                if values.len() != len {
                    None
                } else {
                    Some(values)
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    fn xfconf_display_name() -> &'static str {
        "vec"
    }
}

impl_to_xfconf_value_simple!(bool);
impl_to_xfconf_value_simple!(u8);
impl_to_xfconf_value_short!(u16, crate::XfconfGType::value_from_u16);
impl_to_xfconf_value_simple!(u32);
impl_to_xfconf_value_simple!(u64);
impl_to_xfconf_value_size!(usize, u64, u32, value_from_u16, u8);
impl_to_xfconf_value_simple!(i8);
impl_to_xfconf_value_short!(i16, crate::XfconfGType::value_from_i16);
impl_to_xfconf_value_simple!(i32);
impl_to_xfconf_value_simple!(i64);
impl_to_xfconf_value_size!(isize, i64, i32, value_from_i16, i8);
impl_to_xfconf_value_simple!(f32);
impl_to_xfconf_value_simple!(f64);
impl_to_xfconf_value_simple!(String);
impl_to_xfconf_value_simple!(&str);
impl_to_xfconf_value_simple!(Color);

impl ToXfconfValue for glib::Value {
    fn to_xfconf_value(&self) -> glib::Value {
        self.clone()
    }
}

impl<T: ToXfconfValue> ToXfconfValue for Vec<T> {
    fn to_xfconf_value(&self) -> glib::Value {
        gvalue_slice_to_gvalue(self)
    }
}

impl<T: ToXfconfValue> ToXfconfValue for &[T] {
    fn to_xfconf_value(&self) -> glib::Value {
        gvalue_slice_to_gvalue(self)
    }
}

#[inline]
pub(crate) fn ptr_array_gtype() -> glib::Type {
    unsafe { glib::Type::from_glib(glib::ffi::g_ptr_array_get_type()) }
}

unsafe extern "C" fn gvalue_ptr_free(ptr: *mut c_void) {
    if !ptr.is_null() {
        glib::gobject_ffi::g_value_unset(ptr as *mut _);
        glib::ffi::g_free(ptr);
    }
}

pub(crate) fn gvalue_slice_to_gvalue<T: ToXfconfValue>(values: &[T]) -> glib::Value {
    unsafe {
        let ptrarr = glib::ffi::g_ptr_array_new_full(
            u32::try_from(values.len()).unwrap(),
            Some(gvalue_ptr_free),
        );
        assert!(!ptrarr.is_null());

        for value in values.into_iter() {
            glib::ffi::g_ptr_array_add(ptrarr, value.to_xfconf_value().to_glib_full() as *mut _);
        }

        let value = glib::Value::from_type(ptr_array_gtype());
        glib::gobject_ffi::g_value_take_boxed(value.as_ptr(), ptrarr as *const _);
        value
    }
}
