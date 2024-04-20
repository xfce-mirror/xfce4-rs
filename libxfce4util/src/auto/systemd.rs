// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files.gtk (https://github.com/gtk-rs/gir-files)
// from gir-files.xfce
// DO NOT EDIT

#[cfg(feature = "v4_19_1")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_19_1")))]
use glib::translate::*;

glib::wrapper! {
    ///
    ///
    /// # Implements
    ///
    /// [`trait@glib::ObjectExt`]
    #[doc(alias = "XfceSystemd")]
    pub struct Systemd(Object<ffi::XfceSystemd, ffi::XfceSystemdClass>);

    match fn {
        type_ => || ffi::xfce_systemd_get_type(),
    }
}

impl Systemd {
    /// Check whether systemd can trigger and has authorization for Hibernate.
    ///
    /// # Returns
    ///
    /// [`true`] if the D-Bus request was successful, [`false`] otherwise and `error` is set.
    ///
    /// ## `can_hibernate`
    /// location to store capacity or [`None`]
    ///
    /// ## `auth_hibernate`
    /// location to store authorization or [`None`]
    #[cfg(feature = "v4_19_1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_19_1")))]
    #[doc(alias = "xfce_systemd_can_hibernate")]
    pub fn can_hibernate(&self) -> Result<(Option<bool>, Option<bool>), glib::Error> {
        unsafe {
            let mut can_hibernate = std::mem::MaybeUninit::uninit();
            let mut auth_hibernate = std::mem::MaybeUninit::uninit();
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::xfce_systemd_can_hibernate(
                self.to_glib_none().0,
                can_hibernate.as_mut_ptr(),
                auth_hibernate.as_mut_ptr(),
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok((
                    from_glib(can_hibernate.assume_init()),
                    from_glib(auth_hibernate.assume_init()),
                ))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    /// Check whether systemd can trigger and has authorization for HybridSleep.
    ///
    /// # Returns
    ///
    /// [`true`] if the D-Bus request was successful, [`false`] otherwise and `error` is set.
    ///
    /// ## `can_hybrid_sleep`
    /// location to store capacity or [`None`]
    ///
    /// ## `auth_hybrid_sleep`
    /// location to store authorization or [`None`]
    #[cfg(feature = "v4_19_1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_19_1")))]
    #[doc(alias = "xfce_systemd_can_hybrid_sleep")]
    pub fn can_hybrid_sleep(&self) -> Result<(Option<bool>, Option<bool>), glib::Error> {
        unsafe {
            let mut can_hybrid_sleep = std::mem::MaybeUninit::uninit();
            let mut auth_hybrid_sleep = std::mem::MaybeUninit::uninit();
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::xfce_systemd_can_hybrid_sleep(
                self.to_glib_none().0,
                can_hybrid_sleep.as_mut_ptr(),
                auth_hybrid_sleep.as_mut_ptr(),
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok((
                    from_glib(can_hybrid_sleep.assume_init()),
                    from_glib(auth_hybrid_sleep.assume_init()),
                ))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    /// Check whether systemd can trigger PowerOff.
    ///
    /// # Returns
    ///
    /// [`true`] if the D-Bus request was successful, [`false`] otherwise and `error` is set.
    ///
    /// ## `can_power_off`
    /// location to store capacity or [`None`]
    ///
    /// ## `auth_power_off`
    /// location to store authorization or [`None`]
    #[cfg(feature = "v4_19_1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_19_1")))]
    #[doc(alias = "xfce_systemd_can_power_off")]
    pub fn can_power_off(&self) -> Result<(Option<bool>, Option<bool>), glib::Error> {
        unsafe {
            let mut can_power_off = std::mem::MaybeUninit::uninit();
            let mut auth_power_off = std::mem::MaybeUninit::uninit();
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::xfce_systemd_can_power_off(
                self.to_glib_none().0,
                can_power_off.as_mut_ptr(),
                auth_power_off.as_mut_ptr(),
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok((
                    from_glib(can_power_off.assume_init()),
                    from_glib(auth_power_off.assume_init()),
                ))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    /// Check whether systemd can trigger Reboot.
    ///
    /// # Returns
    ///
    /// [`true`] if the D-Bus request was successful, [`false`] otherwise and `error` is set.
    ///
    /// ## `can_reboot`
    /// location to store capacity or [`None`]
    ///
    /// ## `auth_reboot`
    /// location to store authorization or [`None`]
    #[cfg(feature = "v4_19_1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_19_1")))]
    #[doc(alias = "xfce_systemd_can_reboot")]
    pub fn can_reboot(&self) -> Result<(Option<bool>, Option<bool>), glib::Error> {
        unsafe {
            let mut can_reboot = std::mem::MaybeUninit::uninit();
            let mut auth_reboot = std::mem::MaybeUninit::uninit();
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::xfce_systemd_can_reboot(
                self.to_glib_none().0,
                can_reboot.as_mut_ptr(),
                auth_reboot.as_mut_ptr(),
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok((
                    from_glib(can_reboot.assume_init()),
                    from_glib(auth_reboot.assume_init()),
                ))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    /// Check whether systemd can trigger and has authorization for Suspend.
    ///
    /// # Returns
    ///
    /// [`true`] if the D-Bus request was successful, [`false`] otherwise and `error` is set.
    ///
    /// ## `can_suspend`
    /// location to store capacity or [`None`]
    ///
    /// ## `auth_suspend`
    /// location to store authorization or [`None`]
    #[cfg(feature = "v4_19_1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_19_1")))]
    #[doc(alias = "xfce_systemd_can_suspend")]
    pub fn can_suspend(&self) -> Result<(Option<bool>, Option<bool>), glib::Error> {
        unsafe {
            let mut can_suspend = std::mem::MaybeUninit::uninit();
            let mut auth_suspend = std::mem::MaybeUninit::uninit();
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::xfce_systemd_can_suspend(
                self.to_glib_none().0,
                can_suspend.as_mut_ptr(),
                auth_suspend.as_mut_ptr(),
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok((
                    from_glib(can_suspend.assume_init()),
                    from_glib(auth_suspend.assume_init()),
                ))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    /// Ask systemd to trigger Hibernate.
    /// ## `polkit_interactive`
    /// whether PolicyKit should ask the user to authenticate if needed
    ///
    /// # Returns
    ///
    /// [`true`] if the D-Bus request was successful, [`false`] otherwise and `error` is set.
    #[cfg(feature = "v4_19_1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_19_1")))]
    #[doc(alias = "xfce_systemd_hibernate")]
    pub fn hibernate(&self, polkit_interactive: bool) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::xfce_systemd_hibernate(
                self.to_glib_none().0,
                polkit_interactive.into_glib(),
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

    /// Ask systemd to trigger HybridSleep.
    /// ## `polkit_interactive`
    /// whether PolicyKit should ask the user to authenticate if needed
    ///
    /// # Returns
    ///
    /// [`true`] if the D-Bus request was successful, [`false`] otherwise and `error` is set.
    #[cfg(feature = "v4_19_1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_19_1")))]
    #[doc(alias = "xfce_systemd_hybrid_sleep")]
    pub fn hybrid_sleep(&self, polkit_interactive: bool) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::xfce_systemd_hybrid_sleep(
                self.to_glib_none().0,
                polkit_interactive.into_glib(),
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

    /// Ask systemd to trigger PowerOff.
    /// ## `polkit_interactive`
    /// whether PolicyKit should ask the user to authenticate if needed
    ///
    /// # Returns
    ///
    /// [`true`] if the D-Bus request was successful, [`false`] otherwise and `error` is set.
    #[cfg(feature = "v4_19_1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_19_1")))]
    #[doc(alias = "xfce_systemd_power_off")]
    pub fn power_off(&self, polkit_interactive: bool) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::xfce_systemd_power_off(
                self.to_glib_none().0,
                polkit_interactive.into_glib(),
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

    /// Ask systemd to trigger Reboot.
    /// ## `polkit_interactive`
    /// whether PolicyKit should ask the user to authenticate if needed
    ///
    /// # Returns
    ///
    /// [`true`] if the D-Bus request was successful, [`false`] otherwise and `error` is set.
    #[cfg(feature = "v4_19_1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_19_1")))]
    #[doc(alias = "xfce_systemd_reboot")]
    pub fn reboot(&self, polkit_interactive: bool) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::xfce_systemd_reboot(
                self.to_glib_none().0,
                polkit_interactive.into_glib(),
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

    /// Ask systemd to trigger Suspend.
    /// ## `polkit_interactive`
    /// whether PolicyKit should ask the user to authenticate if needed
    ///
    /// # Returns
    ///
    /// [`true`] if the D-Bus request was successful, [`false`] otherwise and `error` is set.
    #[cfg(feature = "v4_19_1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_19_1")))]
    #[doc(alias = "xfce_systemd_suspend")]
    pub fn suspend(&self, polkit_interactive: bool) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::xfce_systemd_suspend(
                self.to_glib_none().0,
                polkit_interactive.into_glib(),
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

    /// Create a new [`Systemd`][crate::Systemd] instance or increase reference count.
    ///
    /// # Returns
    ///
    /// A reference to the singleton object, to be released with `g_object_unref()`.
    #[cfg(feature = "v4_19_1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_19_1")))]
    #[doc(alias = "xfce_systemd_get")]
    pub fn get() -> Systemd {
        unsafe { from_glib_full(ffi::xfce_systemd_get()) }
    }
}
