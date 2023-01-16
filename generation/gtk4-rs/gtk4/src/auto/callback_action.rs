// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ShortcutAction, Widget};
use glib::translate::*;
use std::{boxed::Box as Box_, fmt};

glib::wrapper! {
    #[doc(alias = "GtkCallbackAction")]
    pub struct CallbackAction(Object<ffi::GtkCallbackAction, ffi::GtkCallbackActionClass>) @extends ShortcutAction;

    match fn {
        type_ => || ffi::gtk_callback_action_get_type(),
    }
}

impl CallbackAction {
    #[doc(alias = "gtk_callback_action_new")]
    pub fn new<P: Fn(&Widget, Option<&glib::Variant>) -> bool + 'static>(
        callback: P,
    ) -> CallbackAction {
        assert_initialized_main_thread!();
        let callback_data: Box_<P> = Box_::new(callback);
        unsafe extern "C" fn callback_func<
            P: Fn(&Widget, Option<&glib::Variant>) -> bool + 'static,
        >(
            widget: *mut ffi::GtkWidget,
            args: *mut glib::ffi::GVariant,
            user_data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let widget = from_glib_borrow(widget);
            let args: Borrowed<Option<glib::Variant>> = from_glib_borrow(args);
            let callback: &P = &*(user_data as *mut _);
            let res = (*callback)(&widget, args.as_ref().as_ref());
            res.into_glib()
        }
        let callback = Some(callback_func::<P> as _);
        unsafe extern "C" fn destroy_func<
            P: Fn(&Widget, Option<&glib::Variant>) -> bool + 'static,
        >(
            data: glib::ffi::gpointer,
        ) {
            let _callback: Box_<P> = Box_::from_raw(data as *mut _);
        }
        let destroy_call2 = Some(destroy_func::<P> as _);
        let super_callback0: Box_<P> = callback_data;
        unsafe {
            from_glib_full(ffi::gtk_callback_action_new(
                callback,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call2,
            ))
        }
    }
}

impl fmt::Display for CallbackAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("CallbackAction")
    }
}
