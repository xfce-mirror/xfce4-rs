use crate::{ffi, Consolekit};
use glib::{translate::*, IsA};

pub trait ConsolekitExtManual {
    #[doc(alias = "xfce_consolekit_can_hibernate")]
    fn can_hibernate(&self) -> Result<(bool, bool), glib::Error>;

    #[doc(alias = "xfce_consolekit_can_hybrid_sleep")]
    fn can_hybrid_sleep(&self) -> Result<(bool, bool), glib::Error>;

    #[doc(alias = "xfce_consolekit_can_power_off")]
    fn can_power_off(&self) -> Result<(bool, bool), glib::Error>;

    #[doc(alias = "xfce_consolekit_can_reboot")]
    fn can_reboot(&self) -> Result<(bool, bool), glib::Error>;

    #[doc(alias = "xfce_consolekit_can_suspend")]
    fn can_suspend(&self) -> Result<(bool, bool), glib::Error>;
}

impl<I: IsA<Consolekit>> ConsolekitExtManual for I {
    #[doc(alias = "xfce_consolekit_can_hibernate")]
    fn can_hibernate(&self) -> Result<(bool, bool), glib::Error> {
        unsafe {
            let mut can_hibernate = std::mem::MaybeUninit::uninit();
            let mut auth_hibernate = std::mem::MaybeUninit::uninit();
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::xfce_consolekit_can_hibernate(
                self.as_ref().to_glib_none().0,
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

    #[doc(alias = "xfce_consolekit_can_hybrid_sleep")]
    fn can_hybrid_sleep(&self) -> Result<(bool, bool), glib::Error> {
        unsafe {
            let mut can_hybrid_sleep = std::mem::MaybeUninit::uninit();
            let mut auth_hybrid_sleep = std::mem::MaybeUninit::uninit();
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::xfce_consolekit_can_hybrid_sleep(
                self.as_ref().to_glib_none().0,
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

    #[doc(alias = "xfce_consolekit_can_power_off")]
    fn can_power_off(&self) -> Result<(bool, bool), glib::Error> {
        unsafe {
            let mut can_power_off = std::mem::MaybeUninit::uninit();
            let mut auth_power_off = std::mem::MaybeUninit::uninit();
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::xfce_consolekit_can_power_off(
                self.as_ref().to_glib_none().0,
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

    #[doc(alias = "xfce_consolekit_can_reboot")]
    fn can_reboot(&self) -> Result<(bool, bool), glib::Error> {
        unsafe {
            let mut can_reboot = std::mem::MaybeUninit::uninit();
            let mut auth_reboot = std::mem::MaybeUninit::uninit();
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::xfce_consolekit_can_reboot(
                self.as_ref().to_glib_none().0,
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

    #[doc(alias = "xfce_consolekit_can_suspend")]
    fn can_suspend(&self) -> Result<(bool, bool), glib::Error> {
        unsafe {
            let mut can_suspend = std::mem::MaybeUninit::uninit();
            let mut auth_suspend = std::mem::MaybeUninit::uninit();
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::xfce_consolekit_can_suspend(
                self.as_ref().to_glib_none().0,
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
}
