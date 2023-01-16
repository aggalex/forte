// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GdkX11Screen")]
    pub struct X11Screen(Object<ffi::GdkX11Screen, ffi::GdkX11ScreenClass>);

    match fn {
        type_ => || ffi::gdk_x11_screen_get_type(),
    }
}

impl X11Screen {
    #[doc(alias = "gdk_x11_screen_get_current_desktop")]
    #[doc(alias = "get_current_desktop")]
    pub fn current_desktop(&self) -> u32 {
        unsafe { ffi::gdk_x11_screen_get_current_desktop(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_x11_screen_get_number_of_desktops")]
    #[doc(alias = "get_number_of_desktops")]
    pub fn number_of_desktops(&self) -> u32 {
        unsafe { ffi::gdk_x11_screen_get_number_of_desktops(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_x11_screen_get_screen_number")]
    #[doc(alias = "get_screen_number")]
    pub fn screen_number(&self) -> i32 {
        unsafe { ffi::gdk_x11_screen_get_screen_number(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_x11_screen_get_window_manager_name")]
    #[doc(alias = "get_window_manager_name")]
    pub fn window_manager_name(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::gdk_x11_screen_get_window_manager_name(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_x11_screen_supports_net_wm_hint")]
    pub fn supports_net_wm_hint(&self, property_name: &str) -> bool {
        unsafe {
            from_glib(ffi::gdk_x11_screen_supports_net_wm_hint(
                self.to_glib_none().0,
                property_name.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "window-manager-changed")]
    pub fn connect_window_manager_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn window_manager_changed_trampoline<F: Fn(&X11Screen) + 'static>(
            this: *mut ffi::GdkX11Screen,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"window-manager-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    window_manager_changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for X11Screen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("X11Screen")
    }
}
