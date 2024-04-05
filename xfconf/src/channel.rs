use std::collections::HashMap;

use crate::Channel;
use glib::{gobject_ffi::GValue, prelude::*, translate::*};

pub trait ChannelExtManual {
    #[doc(alias = "xfconf_channel_set_string_list")]
    fn set_string_list(&self, property: &str, values: &[&str]) -> bool;

    #[doc(alias = "xfconf_channel_get_string_list")]
    fn get_string_list(&self, property: &str) -> Option<Vec<glib::GString>>;

    #[doc(alias = "xfconf_channel_get_properties")]
    fn get_properties(&self, property_base: Option<&str>) -> HashMap<glib::GString, glib::Value>;

    #[doc(alias = "xfconf_channel_get_property")]
    fn get_property(&self, property: &str) -> Option<glib::Value>;
}

impl<O: IsA<Channel>> ChannelExtManual for O {
    // gir seems to think 'values' should be a '&str'
    #[doc(alias = "xfconf_channel_set_string_list")]
    fn set_string_list(&self, property: &str, values: &[&str]) -> bool {
        unsafe {
            from_glib(ffi::xfconf_channel_set_string_list(
                self.as_ref().to_glib_none().0,
                property.to_glib_none().0,
                values.to_glib_none().0,
            ))
        }
    }

    // gir isn't returning an Option
    #[doc(alias = "xfconf_channel_get_string_list")]
    fn get_string_list(&self, property: &str) -> Option<Vec<glib::GString>> {
        unsafe {
            let ptr = ffi::xfconf_channel_get_string_list(
                self.as_ref().to_glib_none().0,
                property.to_glib_none().0,
            );
            if !ptr.is_null() {
                Some(FromGlibPtrContainer::from_glib_full(ptr))
            } else {
                None
            }
        }
    }

    // gir isn't handling a GHashTable return properlya
    #[doc(alias = "xfconf_channel_get_properties")]
    fn get_properties(&self, property_base: Option<&str>) -> HashMap<glib::GString, glib::Value> {
        unsafe {
            let hashtable = ffi::xfconf_channel_get_properties(
                self.as_ref().to_glib_none().0,
                property_base.to_glib_none().0,
            );
            if hashtable.is_null() {
                HashMap::new()
            } else {
                ghashtable_into_hashmap_string_value(hashtable)
            }
        }
    }

    // gir gets the signature right, but then passes the 'value' out parameter incorrectly
    #[doc(alias = "xfconf_channel_get_property")]
    fn get_property(&self, property: &str) -> Option<glib::Value> {
        unsafe {
            let value = std::ptr::null_mut();
            let ret = from_glib(ffi::xfconf_channel_get_property(
                self.as_ref().to_glib_none().0,
                property.to_glib_none().0,
                value,
            ));
            if ret {
                Some(from_glib_none(value))
            } else {
                None
            }
        }
    }
}

unsafe fn ghashtable_into_hashmap_string_value(
    ptr: *mut glib::ffi::GHashTable,
) -> HashMap<glib::GString, glib::Value> {
    unsafe extern "C" fn read_string_hash_table(
        key: glib::ffi::gpointer,
        value: glib::ffi::gpointer,
        hash_map: glib::ffi::gpointer,
    ) -> i32 {
        let key: glib::GString = from_glib_full(key as *const libc::c_char);
        let value: glib::Value = from_glib_full(value as *const GValue);
        let hash_map: &mut HashMap<glib::GString, glib::Value> =
            &mut *(hash_map as *mut HashMap<glib::GString, glib::Value>);
        hash_map.insert(key, value);
        1
    }
    let mut map = HashMap::with_capacity(glib::ffi::g_hash_table_size(ptr) as usize);
    glib::ffi::g_hash_table_foreach_steal(
        ptr,
        Some(read_string_hash_table),
        &mut map as *mut HashMap<glib::GString, glib::Value> as *mut _,
    );
    glib::ffi::g_hash_table_destroy(ptr);
    map
}
