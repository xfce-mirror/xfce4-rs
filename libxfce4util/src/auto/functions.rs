// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files.gtk (https://github.com/gtk-rs/gir-files)
// from gir-files.xfce
// DO NOT EDIT

use glib::translate::*;

/// Creates the shared thumbnail path for the file that corresponds to the given `uri`
/// and `size`. No checks are made regarding the existence of the thumbnail.
///
/// It is the duty of the caller to free the returned string.
/// ## `uri`
/// the uri of the file whose shared thumbnail we want to find.
/// ## `size`
/// the thumbnail size (e.g. normal, large).
///
/// # Returns
///
/// a string with the thumbnail path or NULL if the `uri` could not be converted to
/// a local filename.
#[doc(alias = "xfce_create_shared_thumbnail_path")]
pub fn create_shared_thumbnail_path(uri: &str, size: &str) -> glib::GString {
    unsafe {
        from_glib_full(ffi::xfce_create_shared_thumbnail_path(
            uri.to_glib_none().0,
            size.to_glib_none().0,
        ))
    }
}

//#[doc(alias = "xfce_g_file_create_checksum")]
//pub fn g_file_create_checksum(file: /*Ignored*/&gio::File, cancellable: /*Ignored*/Option<&gio::Cancellable>) -> Result<Option<glib::GString>, glib::Error> {
//    unsafe { TODO: call ffi:xfce_g_file_create_checksum() }
//}

//#[doc(alias = "xfce_g_file_is_trusted")]
//pub fn g_file_is_trusted(file: /*Ignored*/&gio::File, cancellable: /*Ignored*/Option<&gio::Cancellable>) -> Result<(), glib::Error> {
//    unsafe { TODO: call ffi:xfce_g_file_is_trusted() }
//}

//#[doc(alias = "xfce_g_file_metadata_is_supported")]
//pub fn g_file_metadata_is_supported(file: /*Ignored*/&gio::File) -> bool {
//    unsafe { TODO: call ffi:xfce_g_file_metadata_is_supported() }
//}

//#[doc(alias = "xfce_g_file_set_trusted")]
//pub fn g_file_set_trusted(file: /*Ignored*/&gio::File, is_trusted: bool, cancellable: /*Ignored*/Option<&gio::Cancellable>) -> Result<(), glib::Error> {
//    unsafe { TODO: call ffi:xfce_g_file_set_trusted() }
//}

//#[doc(alias = "xfce_g_string_append_quoted")]
//pub fn g_string_append_quoted(string: /*Ignored*/&mut glib::String, unquoted: &str) {
//    unsafe { TODO: call ffi:xfce_g_string_append_quoted() }
//}

/// Similar to [`file_localized()`][crate::file_localized()], but works on directory instead of
/// a file.
/// ## `directory`
/// directory name to check for a localized variant.
///
/// # Returns
///
/// path of the localized directory name or copy of `directory` if
///  no such directory exists. Returned string should be freed using
///  `g_free()`.
#[doc(alias = "xfce_get_dir_localized")]
#[doc(alias = "get_dir_localized")]
pub fn dir_localized(directory: &str) -> glib::GString {
    unsafe { from_glib_full(ffi::xfce_get_dir_localized(directory.to_glib_none().0)) }
}

/// Similar to `xfce_get_file_localized_r`, but works on directory instead
/// of regular file.
/// ## `buffer`
/// destination buffer to store the localized filename to.
/// ## `length`
/// size of `buffer` in bytes.
/// ## `directory`
/// name of directory to check for localized variant of.
///
/// # Returns
///
/// pointer to `buffer` or [`None`] on error.
#[doc(alias = "xfce_get_dir_localized_r")]
#[doc(alias = "get_dir_localized_r")]
pub fn dir_localized_r(buffer: &str, directory: &str) -> glib::GString {
    let length = buffer.len() as _;
    unsafe {
        from_glib_full(ffi::xfce_get_dir_localized_r(
            buffer.to_glib_none().0,
            length,
            directory.to_glib_none().0,
        ))
    }
}

/// Checks if theres a version of `filename` which is localized to the current
/// locale. This is done by appending the full locale name to `filename`, separated
/// by a '.'. If theres no file of that name, it retries using the full locale
/// name without the encoding (if any), then without the qualifier (if any) and
/// at last the base locale is tried. If all of those fails, a copy of `filename`
/// is returned.
/// ## `filename`
/// name of a file to look for a localized version.
///
/// # Returns
///
/// path of the localized file or copy of `filename` if no such
///  file exists. Returned string should be freed using `g_free()`.
#[doc(alias = "xfce_get_file_localized")]
#[doc(alias = "get_file_localized")]
pub fn file_localized(filename: &str) -> glib::GString {
    unsafe { from_glib_full(ffi::xfce_get_file_localized(filename.to_glib_none().0)) }
}

/// Similar in functionality to [`file_localized()`][crate::file_localized()], but stores the
/// result in `buffer` instead of allocating a new buffer.
/// ## `buffer`
/// destination buffer to store the localized filename to.
/// ## `length`
/// size of `buffer` in bytes.
/// ## `filename`
/// name of a file to look for a localized version.
///
/// # Returns
///
/// pointer to `buffer` or [`None`] on error.
#[doc(alias = "xfce_get_file_localized_r")]
#[doc(alias = "get_file_localized_r")]
pub fn file_localized_r(buffer: &str, filename: &str) -> glib::GString {
    let length = buffer.len() as _;
    unsafe {
        from_glib_full(ffi::xfce_get_file_localized_r(
            buffer.to_glib_none().0,
            length,
            filename.to_glib_none().0,
        ))
    }
}

/// Similar to `g_get_home_dir()` in functionality but will never return NULL.
/// While `g_get_home_dir()` may return NULL under certain circumstances, this
/// function is garantied to never ever return NULL, but always return a
/// valid character pointer with the absolute path to the user's home directory.
///
/// The returned string is owned by libxfce4util and must not be freed by
/// the caller.
///
/// # Returns
///
/// the path to the current user's home directory.
#[doc(alias = "xfce_get_homedir")]
#[doc(alias = "get_homedir")]
pub fn homedir() -> glib::GString {
    unsafe { from_glib_none(ffi::xfce_get_homedir()) }
}

//#[doc(alias = "xfce_get_homefile_r")]
//#[doc(alias = "get_homefile_r")]
//pub fn homefile_r(buffer: &str, format: &str, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) -> glib::GString {
//    unsafe { TODO: call ffi:xfce_get_homefile_r() }
//}

//#[doc(alias = "xfce_get_license_text")]
//#[doc(alias = "get_license_text")]
//pub fn license_text(license_type: /*Ignored*/LicenseTextType) -> glib::GString {
//    unsafe { TODO: call ffi:xfce_get_license_text() }
//}

//#[doc(alias = "xfce_get_path_localized")]
//#[doc(alias = "get_path_localized")]
//pub fn path_localized(dst: &str, size: usize, paths: &str, filename: &str, test: /*Ignored*/glib::FileTest) -> glib::GString {
//    unsafe { TODO: call ffi:xfce_get_path_localized() }
//}

/// Safe way to retrieve the path to the user's ".xfce4" directory. The path
/// to the current user's ".xfce4" directory is either taken from the
/// environment variable XFCE4HOME if defined, or if unset, is gained by
/// concatenating the path to the user's home directory and the ".xfce4".
/// That says, it will, by default, return the path "$HOME/.xfce4", where
/// $HOME is replaced with the absolute path to the user's home directory.
///
/// The returned string is managed by libxfce4util and must not be freed by
/// the caller.
///
/// # Returns
///
/// the path to the current user's ".xfce4" directory.
#[doc(alias = "xfce_get_userdir")]
#[doc(alias = "get_userdir")]
pub fn userdir() -> glib::GString {
    unsafe { from_glib_none(ffi::xfce_get_userdir()) }
}

//#[doc(alias = "xfce_get_userfile_r")]
//#[doc(alias = "get_userfile_r")]
//pub fn userfile_r(buffer: &str, format: &str, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) -> glib::GString {
//    unsafe { TODO: call ffi:xfce_get_userfile_r() }
//}

/// Portable way to query the hostname of the node running the process. This
/// function does not ever return [`None`], but always returns a string containing
/// the current node's hostname.
///
/// # Returns
///
/// the current node's hostname. The string has to be freed
///  by the caller using `g_free()`.
#[doc(alias = "xfce_gethostname")]
pub fn gethostname() -> glib::GString {
    unsafe { from_glib_full(ffi::xfce_gethostname()) }
}

/// The locale is of the general form LANG_COUNTRY.ENCODING @ MODIFIER, where
/// each of COUNTRY, ENCODING and MODIFIER can be absent.
///
/// The match is done by actually removing the rightmost element one by one. This
/// is not entirely according to the freedesktop.org specification, but much easier.
/// Will probably be fixed in the future.
/// ## `locale1`
/// the current locale value as returned by setlocale(LC_MESSAGES,[`None`]).
/// ## `locale2`
/// the locale value to match against.
///
/// # Returns
///
/// an integer value indicating the level of matching, where
///  the constant `XFCE_LOCALE_FULL_MATCH` indicates a full match
///  and `XFCE_LOCALE_NO_MATCH` means no match. Every other value
///  indicates a partial match, the higher the value, the better
///  the match. You should not rely on any specific value besides
///  the constants `XFCE_LOCALE_FULL_MATCH` and `XFCE_LOCALE_NO_MATCH`,
///  since the range of returned values may change in the future.
#[doc(alias = "xfce_locale_match")]
pub fn locale_match(locale1: &str, locale2: &str) -> u32 {
    unsafe { ffi::xfce_locale_match(locale1.to_glib_none().0, locale2.to_glib_none().0) }
}

/// Creates the specified directory `whole_path`, but unlike the `mkdir()`
/// function from the standard C library, if any of the parent directories
/// of the `whole_path` do not exists, they are created as well.
///
/// If the directory specified by `whole_path` already exists, this function
/// performs no operation and simply returns [`true`].
/// ## `whole_path`
/// path to the directory to create.
/// ## `mode`
/// file permissions to use for the newly created directories.
///
/// # Returns
///
/// [`true`] on success, else [`false`].
#[doc(alias = "xfce_mkdirhier")]
pub fn mkdirhier(whole_path: &str, mode: libc::c_ulong) -> Result<(), glib::Error> {
    unsafe {
        let mut error = std::ptr::null_mut();
        let is_ok = ffi::xfce_mkdirhier(whole_path.to_glib_none().0, mode, &mut error);
        debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
        if error.is_null() {
            Ok(())
        } else {
            Err(from_glib_full(error))
        }
    }
}

/// Initializes the POSIX signal handler system. Must be called
/// before setting any POSIX signal handlers.
///
/// # Returns
///
/// [`true`] on success, [`false`] on failure, in which case
///  `error` will be set.
#[doc(alias = "xfce_posix_signal_handler_init")]
pub fn posix_signal_handler_init() -> Result<(), glib::Error> {
    unsafe {
        let mut error = std::ptr::null_mut();
        let is_ok = ffi::xfce_posix_signal_handler_init(&mut error);
        debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
        if error.is_null() {
            Ok(())
        } else {
            Err(from_glib_full(error))
        }
    }
}

/// Restores the default handler for `signal`.
/// ## `signal`
/// A POSIX signal id number.
#[doc(alias = "xfce_posix_signal_handler_restore_handler")]
pub fn posix_signal_handler_restore_handler(signal: i32) {
    unsafe {
        ffi::xfce_posix_signal_handler_restore_handler(signal);
    }
}

/// Sets `handler` to be called whenever `signal` is caught by the
/// application. The `user_data` parameter will be passed as an argument
/// to `handler`.
/// ## `signal`
/// A POSIX signal id number.
/// ## `handler`
/// A callback function.
///
/// # Returns
///
/// [`true`] on success, [`false`] otherwise, in which case
///  `error` will be set.
#[doc(alias = "xfce_posix_signal_handler_set_handler")]
pub fn posix_signal_handler_set_handler<P: FnMut(i32)>(
    signal: i32,
    handler: P,
) -> Result<(), glib::Error> {
    let handler_data: P = handler;
    unsafe extern "C" fn handler_func<P: FnMut(i32)>(
        signal: libc::c_int,
        user_data: glib::ffi::gpointer,
    ) {
        let callback = user_data as *mut P;
        (*callback)(signal)
    }
    let handler = Some(handler_func::<P> as _);
    let super_callback0: &P = &handler_data;
    unsafe {
        let mut error = std::ptr::null_mut();
        let is_ok = ffi::xfce_posix_signal_handler_set_handler(
            signal,
            handler,
            super_callback0 as *const _ as *mut _,
            &mut error,
        );
        debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
        if error.is_null() {
            Ok(())
        } else {
            Err(from_glib_full(error))
        }
    }
}

/// Frees all memory associated with the POSIX signal handling system
/// and restores all default signal handlers.
#[doc(alias = "xfce_posix_signal_handler_shutdown")]
pub fn posix_signal_handler_shutdown() {
    unsafe {
        ffi::xfce_posix_signal_handler_shutdown();
    }
}

//#[doc(alias = "xfce_resource_dirs")]
//pub fn resource_dirs(type_: /*Ignored*/ResourceType) -> Vec<glib::GString> {
//    unsafe { TODO: call ffi:xfce_resource_dirs() }
//}

//#[doc(alias = "xfce_resource_lookup")]
//pub fn resource_lookup(type_: /*Ignored*/ResourceType, filename: &str) -> glib::GString {
//    unsafe { TODO: call ffi:xfce_resource_lookup() }
//}

//#[doc(alias = "xfce_resource_lookup_all")]
//pub fn resource_lookup_all(type_: /*Ignored*/ResourceType, filename: &str) -> Vec<glib::GString> {
//    unsafe { TODO: call ffi:xfce_resource_lookup_all() }
//}

//#[doc(alias = "xfce_resource_match")]
//pub fn resource_match(type_: /*Ignored*/ResourceType, pattern: &str, unique: bool) -> Vec<glib::GString> {
//    unsafe { TODO: call ffi:xfce_resource_match() }
//}

//#[doc(alias = "xfce_resource_match_custom")]
//pub fn resource_match_custom<P: FnMut(&str, &str) -> bool>(type_: /*Ignored*/ResourceType, unique: bool, func: P) -> Vec<glib::GString> {
//    unsafe { TODO: call ffi:xfce_resource_match_custom() }
//}

//#[doc(alias = "xfce_resource_pop_path")]
//pub fn resource_pop_path(type_: /*Ignored*/ResourceType) {
//    unsafe { TODO: call ffi:xfce_resource_pop_path() }
//}

//#[doc(alias = "xfce_resource_push_path")]
//pub fn resource_push_path(type_: /*Ignored*/ResourceType, path: &str) {
//    unsafe { TODO: call ffi:xfce_resource_push_path() }
//}

//#[doc(alias = "xfce_resource_save_location")]
//pub fn resource_save_location(type_: /*Ignored*/ResourceType, relpath: &str, create: bool) -> glib::GString {
//    unsafe { TODO: call ffi:xfce_resource_save_location() }
//}

/// Searches `str` for occurances of `pattern` and replaces each
/// such occurance with `replacement`. Returns a newly allocated
/// copy of `str` on which the given replacement were performed.
/// The caller is responsible to free the returned string using
/// `g_free()` when no longer needed.
///
/// Note that `pattern` and `replacement` don't need to be of the
/// same size. If `replacement` is [`None`], the pattern will be
/// removed from the string.
///
/// Note for future Xfce developers: Deprecate this function when
/// `g_string_replace()` is available. (Added since Glib >= 2.68)
/// ## `str`
/// the input string.
/// ## `pattern`
/// a search pattern in `str`.
/// ## `replacement`
/// replacement string for `pattern`.
///
/// # Returns
///
///
///  a newly allocated copy of `str` where all occurrences of
///  `pattern` are replaced with `replacement`. Or [`None`] if
///  `str` is [`None`].
#[doc(alias = "xfce_str_replace")]
pub fn str_replace(str: &str, pattern: &str, replacement: &str) -> Option<glib::GString> {
    unsafe {
        from_glib_full(ffi::xfce_str_replace(
            str.to_glib_none().0,
            pattern.to_glib_none().0,
            replacement.to_glib_none().0,
        ))
    }
}

/// Sets up the translations for `package`.
/// ## `package`
/// the package name.
/// ## `localedir`
/// the `package`<!---->s locale directory.
/// ## `encoding`
/// the encoding to use the `package`<!---->s translations
///  or [`None`] to use "UTF-8".
#[doc(alias = "xfce_textdomain")]
pub fn textdomain(package: &str, localedir: &str, encoding: &str) {
    unsafe {
        ffi::xfce_textdomain(
            package.to_glib_none().0,
            localedir.to_glib_none().0,
            encoding.to_glib_none().0,
        );
    }
}

/// Unescapes sequences in `value` according to Freedesktop.org Desktop Entry Specification.
/// ## `value`
/// Value string to replace escape sequences.
///
/// # Returns
///
/// [`None`] on error, else the string, which should be freed using `g_free()` when
///  no longer needed.
#[doc(alias = "xfce_unescape_desktop_entry_value")]
pub fn unescape_desktop_entry_value(value: &str) -> glib::GString {
    unsafe {
        from_glib_full(ffi::xfce_unescape_desktop_entry_value(
            value.to_glib_none().0,
        ))
    }
}

/// Removes all control characters from `str` up to `end` or up to
/// `max_len` characters (note that characters does not mean bytes with
/// UTF-8), where both `str` and `max_len` may not be given.
///
/// Control characters are replaced in `str` by whitespaces, no new string
/// will be allocated. The operation is done in-place.
/// ## `str`
/// target string.
/// ## `max_len`
/// max characters to check or -1 for no character limit.
/// ## `end`
/// pointer to the endpoint in `str` or [`None`] for no endpoint.
///
/// # Returns
///
/// pointer to `str` or [`None`] on error.
#[doc(alias = "xfce_utf8_remove_controls")]
pub fn utf8_remove_controls(str: &str, end: &str) -> glib::GString {
    let max_len = str.len() as _;
    unsafe {
        from_glib_full(ffi::xfce_utf8_remove_controls(
            str.to_glib_none().0,
            max_len,
            end.to_glib_none().0,
        ))
    }
}

/// Duplicates the `src` string up to `max_len` characters
/// (note that characters does not mean bytes with UTF-8).
///
/// The caller is responsible to free the returned string
/// using `g_free()` when no longer needed.
/// ## `src`
/// target string.
/// ## `max_len`
/// max characters to duplicate or -1 for no character limit.
///
/// # Returns
///
/// pointer to the newly allocated string.
#[doc(alias = "xfce_utf8_strndup")]
pub fn utf8_strndup(src: &str) -> glib::GString {
    let max_len = src.len() as _;
    unsafe { from_glib_full(ffi::xfce_utf8_strndup(src.to_glib_none().0, max_len)) }
}

/// Queries the version string of the installed Xfce desktop environment.
///
/// # Returns
///
/// the overall version information of the installed Xfce desktop.
#[doc(alias = "xfce_version_string")]
pub fn version_string() -> glib::GString {
    unsafe { from_glib_none(ffi::xfce_version_string()) }
}
