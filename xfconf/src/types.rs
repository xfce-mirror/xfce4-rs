use glib::{translate::*, value::ToValue, Type};

use crate::conv::{gvalue_slice_to_gvalue, ptr_array_gtype};

pub trait TryFromXfconfValue: Sized {
    fn try_from_xfconf_value(value: &glib::Value) -> Option<Self>;
    fn xfconf_display_name() -> &'static str;
}

pub trait ToXfconfValue {
    fn to_xfconf_value(&self) -> glib::Value;
}

pub trait XfconfGValueExt {
    #[doc(alias = "xfconf_g_value_get_uint16")]
    fn get_u16(&self) -> Option<u16>;
    #[doc(alias = "xfconf_g_value_get_int16")]
    fn get_i16(&self) -> Option<i16>;
}

impl XfconfGValueExt for glib::Value {
    fn get_u16(&self) -> Option<u16> {
        if self.type_() == XfconfGType::u16() {
            Some(unsafe { crate::ffi::xfconf_g_value_get_uint16(self.as_ptr()) })
        } else {
            None
        }
    }

    fn get_i16(&self) -> Option<i16> {
        if self.type_() == XfconfGType::i16() {
            Some(unsafe { crate::ffi::xfconf_g_value_get_int16(self.as_ptr()) })
        } else {
            None
        }
    }
}

pub struct XfconfGType;

impl XfconfGType {
    #[doc(alias = "xfconf_int16_get_type")]
    pub fn u16() -> glib::Type {
        // SAFETY: xfconf's u16 type is a valid GType
        unsafe { glib::Type::from_glib(ffi::xfconf_uint16_get_type()) }
    }

    #[doc(alias = "xfconf_g_value_set_uint16")]
    pub fn value_from_u16(v: u16) -> glib::Value {
        let value = glib::Value::from_type(Self::u16());
        // SAFETY: 'value' is a valid GValue
        unsafe { crate::ffi::xfconf_g_value_set_uint16(value.as_ptr(), v) };
        value
    }

    #[doc(alias = "xfconf_int16_get_type")]
    pub fn i16() -> glib::Type {
        // SAFETY: xfconf's i16 type is a valid GType
        unsafe { glib::Type::from_glib(ffi::xfconf_int16_get_type()) }
    }

    #[doc(alias = "xfconf_g_value_set_int16")]
    pub fn value_from_i16(v: i16) -> glib::Value {
        let value = glib::Value::from_type(Self::i16());
        // SAFETY: 'value' is a valid GValue
        unsafe { crate::ffi::xfconf_g_value_set_int16(value.as_ptr(), v) };
        value
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub alpha: f64,
}

impl Color {
    pub fn from_value(value: &glib::Value) -> Option<Self> {
        use glib::translate::*;
        use glib::{ffi as glib_ffi, gobject_ffi};

        if value.type_().into_glib() == unsafe { glib_ffi::g_ptr_array_get_type() } {
            eprintln!("it's a gptrarray!");

            let array_ptr = unsafe { gobject_ffi::g_value_get_boxed(value.as_ptr()) }
                as *mut glib_ffi::GPtrArray;
            if !array_ptr.is_null() {
                let values: Vec<glib::Value> =
                    unsafe { FromGlibPtrArrayContainerAsVec::from_glib_none_as_vec(array_ptr) };
                let mut iter = values.iter().fuse();
                let red = iter.next();
                let green = iter.next();
                let blue = iter.next();
                let alpha = iter.next();

                match (red, green, blue, alpha) {
                    (Some(red), Some(green), Some(blue), Some(alpha)) => {
                        if red.type_() == Type::F64
                            && green.type_() == Type::F64
                            && blue.type_() == Type::F64
                            && alpha.type_() == Type::F64
                        {
                            Some(Color {
                                red: red.get().unwrap(),
                                green: green.get().unwrap(),
                                blue: blue.get().unwrap(),
                                alpha: alpha.get().unwrap(),
                            })
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl ToValue for Color {
    fn to_value(&self) -> glib::Value {
        let values = vec![self.red, self.green, self.blue, self.alpha];
        gvalue_slice_to_gvalue(&values)
    }

    fn value_type(&self) -> Type {
        ptr_array_gtype()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_rgba_color_roundtrip() {
        let c = Color {
            red: 1.0,
            green: 0.5,
            blue: 0.25,
            alpha: 0.75,
        };

        let v = c.to_xfconf_value();
        let c1 = Color::try_from_xfconf_value(&v).unwrap();

        assert_eq!(c, c1);
    }
}
