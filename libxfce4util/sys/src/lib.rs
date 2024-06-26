// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files.gtk (https://github.com/gtk-rs/gir-files)
// from gir-files.xfce
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(
    clippy::approx_constant,
    clippy::type_complexity,
    clippy::unreadable_literal,
    clippy::upper_case_acronyms
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

use gio_sys as gio;
use glib_sys as glib;
use gobject_sys as gobject;

#[allow(unused_imports)]
use libc::{
    c_char, c_double, c_float, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void,
    intptr_t, size_t, ssize_t, uintptr_t, FILE,
};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Enums
pub type XfceLicenseTextType = c_int;
pub const XFCE_LICENSE_TEXT_BSD: XfceLicenseTextType = 0;
pub const XFCE_LICENSE_TEXT_GPL: XfceLicenseTextType = 1;
pub const XFCE_LICENSE_TEXT_LGPL: XfceLicenseTextType = 2;

pub type XfceResourceType = c_int;
pub const XFCE_RESOURCE_DATA: XfceResourceType = 0;
pub const XFCE_RESOURCE_CONFIG: XfceResourceType = 1;
pub const XFCE_RESOURCE_CACHE: XfceResourceType = 2;
pub const XFCE_RESOURCE_ICONS: XfceResourceType = 3;
pub const XFCE_RESOURCE_THEMES: XfceResourceType = 4;

// Constants
pub const XFCE_LOCALE_FULL_MATCH: c_int = 50;
pub const XFCE_LOCALE_NO_MATCH: c_int = 0;

// Callbacks
pub type XfceMatchFunc =
    Option<unsafe extern "C" fn(*const c_char, *const c_char, gpointer) -> gboolean>;
pub type XfcePosixSignalHandler = Option<unsafe extern "C" fn(c_int, gpointer)>;

// Records
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XfceConsolekitClass {
    pub parent_class: gobject::GObjectClass,
}

impl ::std::fmt::Debug for XfceConsolekitClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("XfceConsolekitClass @ {self:p}"))
            .field("parent_class", &self.parent_class)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XfceKioskClass {
    pub parent_class: gobject::GObjectClass,
}

impl ::std::fmt::Debug for XfceKioskClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("XfceKioskClass @ {self:p}"))
            .field("parent_class", &self.parent_class)
            .finish()
    }
}

#[repr(C)]
pub struct XfceRc {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for XfceRc {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("XfceRc @ {self:p}")).finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XfceSystemdClass {
    pub parent_class: gobject::GObjectClass,
}

impl ::std::fmt::Debug for XfceSystemdClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("XfceSystemdClass @ {self:p}"))
            .field("parent_class", &self.parent_class)
            .finish()
    }
}

// Classes
#[repr(C)]
pub struct XfceConsolekit {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for XfceConsolekit {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("XfceConsolekit @ {self:p}"))
            .finish()
    }
}

#[repr(C)]
pub struct XfceKiosk {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for XfceKiosk {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("XfceKiosk @ {self:p}")).finish()
    }
}

#[repr(C)]
pub struct XfceSystemd {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for XfceSystemd {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("XfceSystemd @ {self:p}")).finish()
    }
}

#[link(name = "xfce4util")]
extern "C" {

    //=========================================================================
    // XfceRc
    //=========================================================================
    pub fn xfce_rc_get_type() -> GType;
    pub fn xfce_rc_close(rc: *mut XfceRc);
    pub fn xfce_rc_delete_entry(rc: *mut XfceRc, key: *const c_char, global: gboolean);
    pub fn xfce_rc_delete_group(rc: *mut XfceRc, group: *const c_char, global: gboolean);
    pub fn xfce_rc_flush(rc: *mut XfceRc);
    pub fn xfce_rc_get_entries(rc: *const XfceRc, group: *const c_char) -> *mut *mut c_char;
    pub fn xfce_rc_get_group(rc: *const XfceRc) -> *const c_char;
    pub fn xfce_rc_get_groups(rc: *const XfceRc) -> *mut *mut c_char;
    pub fn xfce_rc_get_locale(rc: *const XfceRc) -> *const c_char;
    pub fn xfce_rc_has_entry(rc: *const XfceRc, key: *const c_char) -> gboolean;
    pub fn xfce_rc_has_group(rc: *const XfceRc, group: *const c_char) -> gboolean;
    pub fn xfce_rc_is_dirty(rc: *const XfceRc) -> gboolean;
    pub fn xfce_rc_is_readonly(rc: *const XfceRc) -> gboolean;
    pub fn xfce_rc_read_bool_entry(
        rc: *const XfceRc,
        key: *const c_char,
        fallback: gboolean,
    ) -> gboolean;
    pub fn xfce_rc_read_entry(
        rc: *const XfceRc,
        key: *const c_char,
        fallback: *const c_char,
    ) -> *const c_char;
    pub fn xfce_rc_read_entry_untranslated(
        rc: *const XfceRc,
        key: *const c_char,
        fallback: *const c_char,
    ) -> *const c_char;
    pub fn xfce_rc_read_int_entry(rc: *const XfceRc, key: *const c_char, fallback: c_int) -> c_int;
    pub fn xfce_rc_read_list_entry(
        rc: *const XfceRc,
        key: *const c_char,
        delimiter: *const c_char,
    ) -> *mut *mut c_char;
    pub fn xfce_rc_rollback(rc: *mut XfceRc);
    pub fn xfce_rc_set_group(rc: *mut XfceRc, group: *const c_char);
    pub fn xfce_rc_write_bool_entry(rc: *mut XfceRc, key: *const c_char, value: gboolean);
    pub fn xfce_rc_write_entry(rc: *mut XfceRc, key: *const c_char, value: *const c_char);
    pub fn xfce_rc_write_int_entry(rc: *mut XfceRc, key: *const c_char, value: c_int);
    pub fn xfce_rc_write_list_entry(
        rc: *mut XfceRc,
        key: *const c_char,
        value: *mut *mut c_char,
        separator: *const c_char,
    );
    pub fn xfce_rc_config_open(
        type_: XfceResourceType,
        resource: *const c_char,
        readonly: gboolean,
    ) -> *mut XfceRc;
    pub fn xfce_rc_simple_open(filename: *const c_char, readonly: gboolean) -> *mut XfceRc;

    //=========================================================================
    // XfceConsolekit
    //=========================================================================
    pub fn xfce_consolekit_get_type() -> GType;
    #[cfg(feature = "v4_19_1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_19_1")))]
    pub fn xfce_consolekit_get() -> *mut XfceConsolekit;
    #[cfg(feature = "v4_19_1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_19_1")))]
    pub fn xfce_consolekit_can_hibernate(
        consolekit: *mut XfceConsolekit,
        can_hibernate: *mut gboolean,
        auth_hibernate: *mut gboolean,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    #[cfg(feature = "v4_19_1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_19_1")))]
    pub fn xfce_consolekit_can_hybrid_sleep(
        consolekit: *mut XfceConsolekit,
        can_hybrid_sleep: *mut gboolean,
        auth_hybrid_sleep: *mut gboolean,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    #[cfg(feature = "v4_19_1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_19_1")))]
    pub fn xfce_consolekit_can_power_off(
        consolekit: *mut XfceConsolekit,
        can_power_off: *mut gboolean,
        auth_power_off: *mut gboolean,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    #[cfg(feature = "v4_19_1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_19_1")))]
    pub fn xfce_consolekit_can_reboot(
        consolekit: *mut XfceConsolekit,
        can_reboot: *mut gboolean,
        auth_reboot: *mut gboolean,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    #[cfg(feature = "v4_19_1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_19_1")))]
    pub fn xfce_consolekit_can_suspend(
        consolekit: *mut XfceConsolekit,
        can_suspend: *mut gboolean,
        auth_suspend: *mut gboolean,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    #[cfg(feature = "v4_19_1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_19_1")))]
    pub fn xfce_consolekit_hibernate(
        consolekit: *mut XfceConsolekit,
        polkit_interactive: gboolean,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    #[cfg(feature = "v4_19_1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_19_1")))]
    pub fn xfce_consolekit_hybrid_sleep(
        consolekit: *mut XfceConsolekit,
        polkit_interactive: gboolean,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    #[cfg(feature = "v4_19_1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_19_1")))]
    pub fn xfce_consolekit_power_off(
        consolekit: *mut XfceConsolekit,
        polkit_interactive: gboolean,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    #[cfg(feature = "v4_19_1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_19_1")))]
    pub fn xfce_consolekit_reboot(
        consolekit: *mut XfceConsolekit,
        polkit_interactive: gboolean,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    #[cfg(feature = "v4_19_1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_19_1")))]
    pub fn xfce_consolekit_suspend(
        consolekit: *mut XfceConsolekit,
        polkit_interactive: gboolean,
        error: *mut *mut glib::GError,
    ) -> gboolean;

    //=========================================================================
    // XfceKiosk
    //=========================================================================
    pub fn xfce_kiosk_get_type() -> GType;
    pub fn xfce_kiosk_new(module: *const c_char) -> *mut XfceKiosk;
    pub fn xfce_kiosk_free(kiosk: *mut XfceKiosk);
    pub fn xfce_kiosk_query(kiosk: *const XfceKiosk, capability: *const c_char) -> gboolean;

    //=========================================================================
    // XfceSystemd
    //=========================================================================
    pub fn xfce_systemd_get_type() -> GType;
    #[cfg(feature = "v4_19_1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_19_1")))]
    pub fn xfce_systemd_get() -> *mut XfceSystemd;
    #[cfg(feature = "v4_19_1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_19_1")))]
    pub fn xfce_systemd_can_hibernate(
        systemd: *mut XfceSystemd,
        can_hibernate: *mut gboolean,
        auth_hibernate: *mut gboolean,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    #[cfg(feature = "v4_19_1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_19_1")))]
    pub fn xfce_systemd_can_hybrid_sleep(
        systemd: *mut XfceSystemd,
        can_hybrid_sleep: *mut gboolean,
        auth_hybrid_sleep: *mut gboolean,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    #[cfg(feature = "v4_19_1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_19_1")))]
    pub fn xfce_systemd_can_power_off(
        systemd: *mut XfceSystemd,
        can_power_off: *mut gboolean,
        auth_power_off: *mut gboolean,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    #[cfg(feature = "v4_19_1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_19_1")))]
    pub fn xfce_systemd_can_reboot(
        systemd: *mut XfceSystemd,
        can_reboot: *mut gboolean,
        auth_reboot: *mut gboolean,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    #[cfg(feature = "v4_19_1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_19_1")))]
    pub fn xfce_systemd_can_suspend(
        systemd: *mut XfceSystemd,
        can_suspend: *mut gboolean,
        auth_suspend: *mut gboolean,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    #[cfg(feature = "v4_19_1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_19_1")))]
    pub fn xfce_systemd_hibernate(
        systemd: *mut XfceSystemd,
        polkit_interactive: gboolean,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    #[cfg(feature = "v4_19_1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_19_1")))]
    pub fn xfce_systemd_hybrid_sleep(
        systemd: *mut XfceSystemd,
        polkit_interactive: gboolean,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    #[cfg(feature = "v4_19_1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_19_1")))]
    pub fn xfce_systemd_power_off(
        systemd: *mut XfceSystemd,
        polkit_interactive: gboolean,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    #[cfg(feature = "v4_19_1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_19_1")))]
    pub fn xfce_systemd_reboot(
        systemd: *mut XfceSystemd,
        polkit_interactive: gboolean,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    #[cfg(feature = "v4_19_1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_19_1")))]
    pub fn xfce_systemd_suspend(
        systemd: *mut XfceSystemd,
        polkit_interactive: gboolean,
        error: *mut *mut glib::GError,
    ) -> gboolean;

    //=========================================================================
    // Other functions
    //=========================================================================
    pub fn xfce_append_quoted(string: *mut glib::GString, unquoted: *const c_char);
    pub fn xfce_create_shared_thumbnail_path(
        uri: *const c_char,
        size: *const c_char,
    ) -> *mut c_char;
    pub fn xfce_expand_desktop_entry_field_codes(
        command: *const c_char,
        uri_list: *mut glib::GSList,
        icon: *const c_char,
        name: *const c_char,
        uri: *const c_char,
        requires_terminal: gboolean,
    ) -> *mut c_char;
    pub fn xfce_expand_variables(command: *const c_char, envp: *mut *mut c_char) -> *mut c_char;
    pub fn xfce_g_file_create_checksum(
        file: *mut gio::GFile,
        cancellable: *mut gio::GCancellable,
        error: *mut *mut glib::GError,
    ) -> *mut c_char;
    pub fn xfce_g_file_is_trusted(
        file: *mut gio::GFile,
        cancellable: *mut gio::GCancellable,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn xfce_g_file_metadata_is_supported(file: *mut gio::GFile) -> gboolean;
    pub fn xfce_g_file_set_trusted(
        file: *mut gio::GFile,
        is_trusted: gboolean,
        cancellable: *mut gio::GCancellable,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn xfce_g_string_append_quoted(string: *mut glib::GString, unquoted: *const c_char);
    pub fn xfce_get_dir_localized(directory: *const c_char) -> *mut c_char;
    pub fn xfce_get_dir_localized_r(
        buffer: *mut c_char,
        length: size_t,
        directory: *const c_char,
    ) -> *mut c_char;
    pub fn xfce_get_file_localized(filename: *const c_char) -> *mut c_char;
    pub fn xfce_get_file_localized_r(
        buffer: *mut c_char,
        length: size_t,
        filename: *const c_char,
    ) -> *mut c_char;
    pub fn xfce_get_homedir() -> *const c_char;
    pub fn xfce_get_homefile_r(
        buffer: *mut c_char,
        length: size_t,
        format: *const c_char,
        ...
    ) -> *mut c_char;
    pub fn xfce_get_license_text(license_type: XfceLicenseTextType) -> *const c_char;
    pub fn xfce_get_path_localized(
        dst: *mut c_char,
        size: size_t,
        paths: *const c_char,
        filename: *const c_char,
        test: glib::GFileTest,
    ) -> *mut c_char;
    pub fn xfce_get_userdir() -> *const c_char;
    pub fn xfce_get_userfile_r(
        buffer: *mut c_char,
        length: size_t,
        format: *const c_char,
        ...
    ) -> *mut c_char;
    pub fn xfce_gethostname() -> *mut c_char;
    pub fn xfce_locale_match(locale1: *const c_char, locale2: *const c_char) -> c_uint;
    pub fn xfce_mkdirhier(
        whole_path: *const c_char,
        mode: c_ulong,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn xfce_posix_signal_handler_init(error: *mut *mut glib::GError) -> gboolean;
    pub fn xfce_posix_signal_handler_restore_handler(signal: c_int);
    pub fn xfce_posix_signal_handler_set_handler(
        signal: c_int,
        handler: XfcePosixSignalHandler,
        user_data: gpointer,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn xfce_posix_signal_handler_shutdown();
    pub fn xfce_resource_dirs(type_: XfceResourceType) -> *mut *mut c_char;
    pub fn xfce_resource_lookup(type_: XfceResourceType, filename: *const c_char) -> *mut c_char;
    pub fn xfce_resource_lookup_all(
        type_: XfceResourceType,
        filename: *const c_char,
    ) -> *mut *mut c_char;
    pub fn xfce_resource_match(
        type_: XfceResourceType,
        pattern: *const c_char,
        unique: gboolean,
    ) -> *mut *mut c_char;
    pub fn xfce_resource_match_custom(
        type_: XfceResourceType,
        unique: gboolean,
        func: XfceMatchFunc,
        user_data: gpointer,
    ) -> *mut *mut c_char;
    pub fn xfce_resource_pop_path(type_: XfceResourceType);
    pub fn xfce_resource_push_path(type_: XfceResourceType, path: *const c_char);
    pub fn xfce_resource_save_location(
        type_: XfceResourceType,
        relpath: *const c_char,
        create: gboolean,
    ) -> *mut c_char;
    pub fn xfce_str_replace(
        str: *const c_char,
        pattern: *const c_char,
        replacement: *const c_char,
    ) -> *mut c_char;
    pub fn xfce_textdomain(
        package: *const c_char,
        localedir: *const c_char,
        encoding: *const c_char,
    );
    pub fn xfce_unescape_desktop_entry_value(value: *const c_char) -> *mut c_char;
    pub fn xfce_utf8_remove_controls(
        str: *mut c_char,
        max_len: ssize_t,
        end: *const c_char,
    ) -> *mut c_char;
    pub fn xfce_utf8_strndup(src: *const c_char, max_len: ssize_t) -> *mut c_char;
    pub fn xfce_version_string() -> *const c_char;

}
