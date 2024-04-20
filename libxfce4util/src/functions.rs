// FIXME

// use glib::{gobject_ffi::GValue, prelude::*, translate::*};

// #[doc(alias = "xfce_expand_desktop_entry_field_codes")]
// fn expand_desktop_entry_field_codes(
//     command: &str,
//     uri_list: &[&str],
//     icon: &str,
//     name: &str,
//     uri: &str,
//     requires_terminal: bool,
// ) -> glib::GString {
//     unsafe {
//         from_glib_full(ffi::xfce_expand_desktop_entry_field_codes(
//             command.to_glib_none().0,
//             uri_list.to_glib_none().0,
//             icon.to_glib_none().0,
//             name.to_glib_none().0,
//             uri.to_glib_none().0,
//             requires_terminal.into_glib(),
//         ))
//     }
// }

// #[doc(alias = "xfce_expand_variables")]
// fn expand_variables(command: &str, envp: &str) -> glib::GString {
//     unsafe {
//         from_glib_full(ffi::xfce_expand_variables(
//             command.to_glib_none().0,
//             envp.to_glib_none().0,
//         ))
//     }
// }
