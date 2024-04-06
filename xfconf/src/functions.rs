/// Shuts down and frees any resources consumed by the Xfconf library.
/// If [`init()`][crate::init()] is called multiple times, [`shutdown()`][crate::shutdown()] must be
/// called an equal number of times to shut down the library.
#[doc(alias = "xfconf_shutdown")]
pub unsafe fn shutdown() {
    ffi::xfconf_shutdown();
}
