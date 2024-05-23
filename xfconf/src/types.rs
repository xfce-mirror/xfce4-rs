use glib::{translate::FromGlib, value::ToValue, Type};

macro_rules! impl_try_from_xfconf_value_int {
    ($ty:ty) => {
        impl TryFromXfconfValue for $ty {
            fn try_from_xfconf_value(value: glib::Value) -> Option<Self> {
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
                    x if x == crate::XfconfGType::u16() => value
                        .get::<u32>()
                        .ok()
                        .and_then(|v| <$ty>::try_from(v).ok()),
                    x if x == crate::XfconfGType::i16() => value
                        .get::<i32>()
                        .ok()
                        .and_then(|v| <$ty>::try_from(v).ok()),
                    _ => None,
                }
            }

            fn xfconf_display_name() -> &'static str {
                stringify!($ty)
            }
        }
    };
}

macro_rules! impl_try_from_xfconf_value_float {
    ($ty:ty, $gty:ident) => {
        impl TryFromXfconfValue for $ty {
            fn try_from_xfconf_value(value: glib::Value) -> Option<Self> {
                match value.type_() {
                    Type::$gty => value.get::<$ty>().ok(),
                    Type::STRING => value.get::<&str>().ok().and_then(|v| v.parse::<$ty>().ok()),
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
    ($ty:ty, $t64:ty, $t32:ty) => {
        impl ToXfconfValue for $ty {
            fn to_xfconf_value(&self) -> glib::Value {
                let ty_size = std::mem::size_of::<$ty>();
                if ty_size == std::mem::size_of::<$t64>() {
                    (*self as $t64).to_value()
                } else if ty_size == std::mem::size_of::<$t32>() {
                    (*self as $t32).to_value()
                } else {
                    panic!("Only 32- and 64-bit systems supported");
                }
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

pub trait TryFromXfconfValue: Sized {
    fn try_from_xfconf_value(value: glib::Value) -> Option<Self>;
    fn xfconf_display_name() -> &'static str;
}

pub trait ToXfconfValue {
    fn to_xfconf_value(&self) -> glib::Value;
}

pub struct XfconfGType;

impl XfconfGType {
    pub fn u16() -> glib::Type {
        // SAFETY: xfconf's u16 type is a valid GType
        unsafe { glib::Type::from_glib(ffi::xfconf_uint16_get_type()) }
    }

    pub fn value_from_u16(v: u16) -> glib::Value {
        let value = glib::Value::from_type(Self::u16());
        // SAFETY: 'value' is a valid GValue
        unsafe { crate::ffi::xfconf_g_value_set_uint16(value.as_ptr(), v) };
        value
    }

    pub fn i16() -> glib::Type {
        // SAFETY: xfconf's i16 type is a valid GType
        unsafe { glib::Type::from_glib(ffi::xfconf_int16_get_type()) }
    }

    pub fn value_from_i16(v: i16) -> glib::Value {
        let value = glib::Value::from_type(Self::i16());
        // SAFETY: 'value' is a valid GValue
        unsafe { crate::ffi::xfconf_g_value_set_int16(value.as_ptr(), v) };
        value
    }
}

impl_try_from_xfconf_value_int!(u8);
impl_try_from_xfconf_value_int!(u16);
impl_try_from_xfconf_value_int!(u32);
impl_try_from_xfconf_value_int!(u64);
impl_try_from_xfconf_value_int!(i8);
impl_try_from_xfconf_value_int!(i16);
impl_try_from_xfconf_value_int!(i32);
impl_try_from_xfconf_value_int!(i64);
impl_try_from_xfconf_value_float!(f32, F32);
impl_try_from_xfconf_value_float!(f64, F64);

impl TryFromXfconfValue for bool {
    fn try_from_xfconf_value(value: glib::Value) -> Option<Self> {
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

impl TryFromXfconfValue for String {
    fn try_from_xfconf_value(value: glib::Value) -> Option<Self> {
        value.get::<String>().ok()
    }

    fn xfconf_display_name() -> &'static str {
        "string"
    }
}

impl_to_xfconf_value_simple!(bool);
impl_to_xfconf_value_simple!(u8);
impl_to_xfconf_value_simple!(u32);
impl_to_xfconf_value_simple!(u64);
impl_to_xfconf_value_simple!(i8);
impl_to_xfconf_value_simple!(i32);
impl_to_xfconf_value_simple!(i64);
impl_to_xfconf_value_simple!(f32);
impl_to_xfconf_value_simple!(f64);
impl_to_xfconf_value_size!(usize, u64, u32);
impl_to_xfconf_value_size!(isize, i64, i32);
impl_to_xfconf_value_short!(u16, crate::XfconfGType::value_from_u16);
impl_to_xfconf_value_short!(i16, crate::XfconfGType::value_from_i16);
